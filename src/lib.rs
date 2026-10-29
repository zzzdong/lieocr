//! lieocr —— 纯 Rust 文档 OCR 核心库。
//!
//! 分层：
//! - [`pipeline`]：端到端入口，图片 → 版面 → 识别 → 文档树（模型只在这里加载一次）
//! - [`layout`] / [`ocr`] / [`table`] / [`preprocess`]：单步能力（版面检测、行识别、表格、图像预处理）
//! - [`document`]：与渲染无关的文档结构树
//! - [`render`]：文档树 → Markdown / 纯文本，`Renderer` trait 可扩展 PDF、DOCX
//! - [`server`]：HTTP 服务（独立 HTTP 线程 + 多线程推理 worker 池）
//!
//! ```no_run
//! use lieocr::{Pipeline, MarkdownRenderer, Renderer};
//!
//! let rgb = lieocr::decode_image(&std::fs::read("test.png").unwrap()).unwrap();
//! let doc = Pipeline::new().analyze(&rgb, true);
//! println!("{}", MarkdownRenderer.render(&doc));
//! ```

pub mod document;
pub mod layout;
pub mod model;
pub mod ocr;
pub mod pipeline;
pub mod preprocess;
pub mod render;
pub mod server;
pub mod table;

pub use document::{Block, Document, ImageData, Item, Node, NodeKind, Rect};
pub use layout::{LABELS, Region};
pub use model::set_models_dir;
pub use pipeline::Pipeline;
pub use render::Renderer;
pub use render::markdown::MarkdownRenderer;
pub use render::plain::PlainRenderer;

use image::RgbImage;

/// 从内存字节解码图片为 RGB8（HTTP 上传、内存图片等场景）。
pub fn decode_image(bytes: &[u8]) -> Result<RgbImage, image::ImageError> {
    Ok(image::load_from_memory(bytes)?.to_rgb8())
}
