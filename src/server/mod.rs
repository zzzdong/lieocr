//! HTTP 服务：独立 HTTP 线程 + 多线程推理 worker 池。
//!
//! ```text
//! POST /ocr?format=json|markdown|text&layout=true|false   multipart 字段 file
//! GET  /health
//! ```
//!
//! 线程模型：
//! - `http` 线程：自带 tokio runtime，只做请求解析、参数校验、响应编码，不碰推理
//! - `ocr-worker-*` 线程：N 个，从有界队列取任务做推理，结果通过 oneshot 回传
//! - 主线程：调用 [`serve`] 后阻塞等待 HTTP 线程结束，随后回收 worker 池

mod pool;

pub use pool::{SubmitError, WorkerPool};

use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

use axum::extract::{Multipart, Query, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};

use crate::document::{Document, NodeKind};
use crate::layout::LABELS;
use crate::render::Renderer;
use crate::render::markdown::MarkdownRenderer;
use crate::render::plain::PlainRenderer;

/// 服务配置。
#[derive(Clone, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    /// 推理 worker 线程数，`0` = 按 CPU 核数自动
    pub workers: usize,
    /// 队列积压上限，`0` = `workers * 2`
    pub queue: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_owned(),
            port: 8080,
            workers: 0,
            queue: 0,
        }
    }
}

struct AppState {
    pool: Arc<WorkerPool>,
}

/// 启动服务并阻塞到退出（阻塞调用，通常在 `main` 里直接用）。
///
/// 模型在这里加载一次，之后所有 worker 共享；HTTP 线程只处理协议层。
pub fn serve(config: Config) -> std::io::Result<()> {
    println!("加载模型中…");
    let pool = Arc::new(WorkerPool::new(config.workers, config.queue)?);
    println!("模型就绪，{} 个推理 worker", pool.workers());

    let addr = format!("{}:{}", config.host, config.port);
    let http = thread::Builder::new()
        .name("http".to_owned())
        .spawn(move || http_main(addr, pool))?;

    match http.join() {
        Ok(result) => result,
        Err(_) => Err(std::io::Error::other("HTTP 线程异常退出")),
    }
}

fn http_main(addr: String, pool: Arc<WorkerPool>) -> std::io::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()?;

    rt.block_on(async move {
        let app = Router::new()
            .route("/health", get(health))
            .route("/ocr", post(ocr))
            .with_state(Arc::new(AppState { pool }));

        let listener = tokio::net::TcpListener::bind(&addr).await?;
        println!(
            "listening on http://{addr}\n  POST /ocr     multipart: file=<图片>; query: format=json|markdown|text, layout=true|false\n  GET  /health"
        );
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await
    })
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    println!("\n收到退出信号，正在关闭服务…");
}

async fn health() -> &'static str {
    "ok"
}

async fn ocr(
    State(state): State<Arc<AppState>>,
    Query(query): Query<HashMap<String, String>>,
    mut multipart: Multipart,
) -> Response {
    let format = query
        .get("format")
        .map(|s| s.to_ascii_lowercase())
        .unwrap_or_else(|| "json".to_owned());
    let use_layout = query
        .get("layout")
        .map(|v| v != "false" && v != "0")
        .unwrap_or(true);

    let mut bytes: Option<Vec<u8>> = None;
    loop {
        match multipart.next_field().await {
            Ok(Some(field)) => {
                let name = field.name().unwrap_or_default().to_owned();
                if name == "file" || name == "image" {
                    match field.bytes().await {
                        Ok(b) => bytes = Some(b.to_vec()),
                        Err(e) => {
                            return fail(StatusCode::BAD_REQUEST, format!("读取上传内容失败: {e}"));
                        }
                    }
                }
            }
            Ok(None) => break,
            Err(e) => {
                return fail(StatusCode::BAD_REQUEST, format!("解析 multipart 失败: {e}"));
            }
        }
    }

    let Some(bytes) = bytes else {
        return fail(
            StatusCode::BAD_REQUEST,
            "缺少 multipart 字段 file".to_owned(),
        );
    };
    let img = match crate::decode_image(&bytes) {
        Ok(img) => img,
        Err(e) => return fail(StatusCode::BAD_REQUEST, format!("无法解码图片: {e}")),
    };

    // 交给 worker 池：这里只等结果，不占用推理算力
    let rx = match state.pool.submit(img, use_layout) {
        Ok(rx) => rx,
        Err(e) => {
            return fail(StatusCode::SERVICE_UNAVAILABLE, format!("{e}，请稍后重试"));
        }
    };
    let doc = match rx.await {
        Ok(Ok(doc)) => doc,
        Ok(Err(msg)) => {
            return fail(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("推理失败: {msg}"),
            );
        }
        Err(_) => {
            return fail(
                StatusCode::INTERNAL_SERVER_ERROR,
                "推理失败: worker 已退出".to_owned(),
            );
        }
    };

    match format.as_str() {
        "markdown" | "md" => (
            [(header::CONTENT_TYPE, "text/markdown; charset=utf-8")],
            MarkdownRenderer.render(&doc),
        )
            .into_response(),
        "text" | "plain" => (
            [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
            PlainRenderer.render(&doc),
        )
            .into_response(),
        _ => Json(document_json(&doc)).into_response(),
    }
}

fn fail(status: StatusCode, msg: String) -> Response {
    (status, Json(serde_json::json!({ "error": msg }))).into_response()
}

fn document_json(doc: &Document) -> serde_json::Value {
    let nodes: Vec<serde_json::Value> = doc
        .walk()
        .iter()
        .map(|n| {
            serde_json::json!({
                "kind": kind_name(&n.kind),
                "label": LABELS.get(n.label).copied().unwrap_or("unknown"),
                "score": (n.score as f64 * 1000.0).round() / 1000.0,
                "bbox": [n.bbox.x0, n.bbox.y0, n.bbox.x1, n.bbox.y1],
            })
        })
        .collect();

    serde_json::json!({
        "width": doc.width,
        "height": doc.height,
        "markdown": MarkdownRenderer.render(doc),
        "text": PlainRenderer.render(doc),
        "nodes": nodes,
    })
}

fn kind_name(kind: &NodeKind) -> &'static str {
    match kind {
        NodeKind::Heading { .. } => "heading",
        NodeKind::Section { .. } => "section",
        NodeKind::Paragraph { .. } => "paragraph",
        NodeKind::Figure { .. } => "figure",
        NodeKind::Table { .. } => "table",
        NodeKind::Formula { .. } => "formula",
        NodeKind::Decoration { .. } => "decoration",
    }
}
