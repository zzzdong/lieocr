//! HTTP 服务：上传图片 → 返回识别结果。
//!
//! ```text
//! POST /ocr?format=json|markdown|text&layout=true|false   multipart 字段 file
//! GET  /health
//! ```

use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::{Multipart, Query, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use tokio::sync::Mutex;

use crate::document::{Document, NodeKind};
use crate::layout::LABELS;
use crate::pipeline::Pipeline;
use crate::render::Renderer;
use crate::render::markdown::MarkdownRenderer;
use crate::render::plain::PlainRenderer;

struct AppState {
    /// 推理是 CPU 密集的，串行化避免多请求互相拖垮
    pipeline: Mutex<Pipeline>,
}

pub async fn serve(host: String, port: u16) -> std::io::Result<()> {
    println!("加载模型中…");
    let state = Arc::new(AppState {
        pipeline: Mutex::new(Pipeline::new()),
    });

    let app = Router::new()
        .route("/health", get(health))
        .route("/ocr", post(ocr))
        .with_state(state);

    let addr = format!("{host}:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!(
        "listening on http://{addr}\n  POST /ocr     multipart: file=<图片>; query: format=json|markdown|text, layout=true|false\n  GET  /health"
    );
    axum::serve(listener, app).await
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
                        Err(e) => return bad_request(format!("读取上传内容失败: {e}")),
                    }
                }
            }
            Ok(None) => break,
            Err(e) => return bad_request(format!("解析 multipart 失败: {e}")),
        }
    }

    let Some(bytes) = bytes else {
        return bad_request("缺少 multipart 字段 file".to_owned());
    };
    let img = match image::load_from_memory(&bytes) {
        Ok(img) => img.to_rgb8(),
        Err(e) => return bad_request(format!("无法解码图片: {e}")),
    };

    let state = state.clone();
    let doc = match tokio::task::spawn_blocking(move || {
        let pipeline = state.pipeline.blocking_lock();
        pipeline.analyze(&img, use_layout)
    })
    .await
    {
        Ok(doc) => doc,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("推理失败: {e}") })),
            )
                .into_response();
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

fn bad_request(msg: String) -> Response {
    (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": msg }))).into_response()
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
