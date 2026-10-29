//! Markdown 渲染。图片以 base64 data URI 内嵌，表格渲染成 GFM 表格。

use std::fmt::Write;
use std::io::Cursor;

use crate::document::{Document, ImageData, Node, NodeKind};
use crate::render::Renderer;

pub struct MarkdownRenderer;

impl Renderer for MarkdownRenderer {
    type Output = String;

    fn render(&self, doc: &Document) -> String {
        let mut out = String::new();
        for node in &doc.children {
            render_node(node, &mut out);
        }
        out
    }
}

fn render_node(node: &Node, out: &mut String) {
    match &node.kind {
        NodeKind::Heading { level, text } => {
            if text.is_empty() {
                return;
            }
            blank(out);
            let _ = writeln!(out, "{} {}", "#".repeat(*level as usize), text);
        }
        NodeKind::Section {
            level,
            title,
            children,
        } => {
            if !title.is_empty() {
                blank(out);
                let _ = writeln!(out, "{} {}", "#".repeat(*level as usize), title);
            }
            for child in children {
                render_node(child, out);
            }
        }
        NodeKind::Paragraph { text } => {
            if text.is_empty() {
                return;
            }
            blank(out);
            let _ = writeln!(out, "{text}");
        }
        NodeKind::Figure { caption, image } => {
            blank(out);
            match image {
                Some(img) => {
                    // 图注已作为 alt 文本，不再重复输出
                    let alt = caption.as_deref().unwrap_or("image");
                    let b64 = base64(&png_bytes(img));
                    let _ = writeln!(out, "![{alt}](data:image/png;base64,{b64})");
                }
                None => {
                    out.push_str("> [图片]");
                    push_caption(caption, out);
                }
            }
        }
        NodeKind::Table { caption, rows } => {
            blank(out);
            if rows.is_empty() {
                out.push_str("> [表格：未能还原]");
            } else {
                render_table(rows, out);
            }
            push_caption(caption, out);
        }
        NodeKind::Formula { caption } => {
            blank(out);
            out.push_str("> [公式：未识别]");
            push_caption(caption, out);
        }
        // 页眉/页脚/页码不进入正文
        NodeKind::Decoration { .. } => {}
    }
}

/// 保证块与块之间恰好一个空行。
fn blank(out: &mut String) {
    if out.is_empty() {
        return;
    }
    if !out.ends_with('\n') {
        out.push('\n');
    }
    if !out.ends_with("\n\n") {
        out.push('\n');
    }
}

fn render_table(rows: &[Vec<String>], out: &mut String) {
    let cols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    if cols == 0 {
        return;
    }
    for (i, row) in rows.iter().enumerate() {
        out.push('|');
        for c in 0..cols {
            let cell = row
                .get(c)
                .map(|s| s.replace('|', "\\|").replace(['\n', '\r'], " "))
                .unwrap_or_default();
            let _ = write!(out, " {} |", cell.trim());
        }
        out.push('\n');
        if i == 0 {
            out.push('|');
            for _ in 0..cols {
                out.push_str(" --- |");
            }
            out.push('\n');
        }
    }
}

fn push_caption(caption: &Option<String>, out: &mut String) {
    if let Some(c) = caption {
        if !c.is_empty() {
            let _ = writeln!(out, "\n*{c}*");
        }
    }
}

/// 图片区域 → PNG 字节。
fn png_bytes(img: &ImageData) -> Vec<u8> {
    let mut buf = Vec::new();
    if let Some(rgb) = image::RgbImage::from_raw(img.width, img.height, img.rgb.clone()) {
        let _ = rgb.write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png);
    }
    buf
}

/// 标准 base64（含 padding），避免为了内嵌图片引入额外依赖。
fn base64(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = *chunk.get(1).unwrap_or(&0) as u32;
        let b2 = *chunk.get(2).unwrap_or(&0) as u32;
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(TABLE[(n >> 18) as usize & 63] as char);
        out.push(TABLE[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 {
            TABLE[(n >> 6) as usize & 63] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            TABLE[n as usize & 63] as char
        } else {
            '='
        });
    }
    out
}
