//! 渲染层：文档树 → 具体格式。
//!
//! 目前实现了 Markdown 与纯文本；PDF / DOCX 只需新增一个 `Renderer` 实现，
//! 因为 `Document` 保留了 bbox、类别、层级等信息。

pub mod markdown;
pub mod plain;

use crate::document::Document;

pub trait Renderer {
    type Output;

    fn render(&self, doc: &Document) -> Self::Output;
}
