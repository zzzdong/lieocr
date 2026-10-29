mod document;
mod layout;
mod model;
mod ocr;
mod pipeline;
mod preprocess;
mod render;
mod server;
mod table;

use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

use crate::document::Document;
use crate::pipeline::Pipeline;
use crate::render::Renderer;
use crate::render::markdown::MarkdownRenderer;
use crate::render::plain::PlainRenderer;

#[derive(Parser)]
#[command(
    name = "lieocr",
    version,
    about = "版面分析与 OCR 文档结构化（PP-DocLayout + PP-OCRv6，纯 Rust / burn）",
    args_conflicts_with_subcommands = true
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    #[command(flatten)]
    model: ModelArgs,

    /// 图片路径（省略子命令时直接识别）
    image: Option<PathBuf>,
    /// 输出 Markdown
    #[arg(short = 'm', long)]
    markdown: bool,
    /// 结果写入文件
    #[arg(short = 'o', long)]
    output: Option<PathBuf>,
    /// 跳过版面模型，整页当成一块文本
    #[arg(long)]
    no_layout: bool,
}

#[derive(clap::Args, Clone)]
struct ModelArgs {
    /// 模型权重目录（默认：可执行文件同级 `models/`，其次当前目录 `models/`）
    #[arg(long, value_name = "DIR")]
    models_dir: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Command {
    /// 识别单张图片
    Ocr {
        #[command(flatten)]
        model: ModelArgs,
        /// 图片路径
        image: PathBuf,
        /// 输出 Markdown
        #[arg(short = 'm', long)]
        markdown: bool,
        /// 结果写入文件
        #[arg(short = 'o', long)]
        output: Option<PathBuf>,
        /// 跳过版面模型，整页当成一块文本
        #[arg(long)]
        no_layout: bool,
    },
    /// 启动 HTTP 服务
    Server {
        #[command(flatten)]
        model: ModelArgs,
        /// 监听地址
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        /// 监听端口
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
}

fn main() {
    let cli = Cli::parse();
    // --models-dir 优先于 LIEOCR_MODEL_DIR
    let models_dir = match &cli.command {
        Some(Command::Ocr { model, .. }) | Some(Command::Server { model, .. }) => {
            model.models_dir.clone()
        }
        None => cli.model.models_dir.clone(),
    };
    model::set_models_dir(models_dir);

    match cli.command {
        Some(Command::Server { host, port, .. }) => {
            // 推理线程需要大栈：burn 生成的模型代码栈帧很大
            let rt = match tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .thread_stack_size(64 << 20)
                .build()
            {
                Ok(rt) => rt,
                Err(e) => {
                    eprintln!("创建 runtime 失败: {e}");
                    std::process::exit(1);
                }
            };
            if let Err(e) = rt.block_on(server::serve(host, port)) {
                eprintln!("服务异常: {e}");
                std::process::exit(1);
            }
        }
        Some(Command::Ocr {
            image,
            markdown,
            output,
            no_layout,
            ..
        }) => run_ocr(&image, markdown, output.as_deref(), no_layout),
        None => {
            let image = cli.image.unwrap_or_else(|| PathBuf::from("test.png"));
            run_ocr(&image, cli.markdown, cli.output.as_deref(), cli.no_layout);
        }
    }
}

fn run_ocr(path: &Path, markdown: bool, out: Option<&Path>, no_layout: bool) {
    let rgb = image::open(path)
        .unwrap_or_else(|e| panic!("无法打开图片 {}: {e}", path.display()))
        .to_rgb8();
    // 直接输出 Markdown 到 stdout 时，不能让日志混进文档
    let info = !markdown || out.is_some();
    if info {
        println!("图片: {} ({}x{})\n", path.display(), rgb.width(), rgb.height());
    }

    let doc = Pipeline::new().analyze(&rgb, !no_layout);

    if info {
        println!("===== 结构树（{} 个顶层节点）=====", doc.children.len());
        print_tree(&doc, 0);
    }

    let output = if markdown {
        MarkdownRenderer.render(&doc)
    } else {
        PlainRenderer.render(&doc)
    };

    match out {
        Some(p) => {
            std::fs::write(p, &output)
                .unwrap_or_else(|e| panic!("写入 {} 失败: {e}", p.display()));
            println!("\n已写入 {}（{} 字符）", p.display(), output.chars().count());
        }
        None if markdown => print!("{output}"),
        None => println!("\n===== OCR 结果 =====\n{output}"),
    }
}

/// 调试用：把文档树打印成缩进大纲。
fn print_tree(doc: &Document, depth: usize) {
    fn visit(nodes: &[crate::document::Node], depth: usize) {
        for n in nodes {
            let pad = "  ".repeat(depth);
            let desc = match &n.kind {
                crate::document::NodeKind::Heading { level, text } => {
                    format!("h{level}: {text}")
                }
                crate::document::NodeKind::Section { level, title, .. } => {
                    format!("section(h{level}): {title}")
                }
                crate::document::NodeKind::Paragraph { text } => {
                    let brief: String = text.chars().take(30).collect();
                    format!(
                        "p: {brief}{}",
                        if text.chars().count() > 30 { "…" } else { "" }
                    )
                }
                crate::document::NodeKind::Figure { image, .. } => match image {
                    Some(i) => format!("figure {}x{}", i.width, i.height),
                    None => "figure".to_owned(),
                },
                crate::document::NodeKind::Table { rows, .. } => format!(
                    "table {}x{}",
                    rows.len(),
                    rows.first().map(|r| r.len()).unwrap_or(0)
                ),
                crate::document::NodeKind::Formula { .. } => "formula".to_owned(),
                crate::document::NodeKind::Decoration { .. } => "decoration".to_owned(),
            };
            println!(
                "{pad}- {desc}  [{} {:.2} {}x{} @{},{},{},{}]",
                crate::layout::LABELS.get(n.label).copied().unwrap_or("?"),
                n.score,
                n.bbox.width(),
                n.bbox.height(),
                n.bbox.x0,
                n.bbox.y0,
                n.bbox.x1,
                n.bbox.y1
            );
            visit(n.children(), depth + 1);
        }
    }
    visit(&doc.children, depth);
}
