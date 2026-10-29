//! 纯文本渲染：每块一行，带版面类别前缀，便于调试与人工核对。

use std::fmt::Write;

use crate::document::{Document, Node, NodeKind};
use crate::layout::LABELS;
use crate::render::Renderer;

pub struct PlainRenderer;

impl Renderer for PlainRenderer {
    type Output = String;

    fn render(&self, doc: &Document) -> String {
        let mut out = String::new();
        render_nodes(&doc.children, &mut out);
        out
    }
}

fn render_nodes(nodes: &[Node], out: &mut String) {
    for node in nodes {
        match &node.kind {
            NodeKind::Section {
                title, children, ..
            } => {
                line(out, node.label, title);
                render_nodes(children, out);
            }
            NodeKind::Heading { text, .. } | NodeKind::Paragraph { text } => {
                line(out, node.label, text);
            }
            NodeKind::Figure { caption, image } => {
                let name = label_name(node.label);
                match image {
                    Some(img) => {
                        let _ = writeln!(out, "[{name}] <图片 {}x{}>", img.width, img.height);
                    }
                    None => {
                        let _ = writeln!(out, "[{name}] <跳过>");
                    }
                }
                append_caption(caption, out);
            }
            NodeKind::Table { caption, rows } => {
                let name = label_name(node.label);
                if rows.is_empty() {
                    let _ = writeln!(out, "[{name}] <未能还原>");
                } else {
                    let cols = rows.first().map(|r| r.len()).unwrap_or(0);
                    let _ = writeln!(out, "[{name}] <{} 行 x {cols} 列>", rows.len());
                    for row in rows {
                        let _ = writeln!(out, "    | {} |", row.join(" | "));
                    }
                }
                append_caption(caption, out);
            }
            NodeKind::Formula { caption } => {
                let _ = writeln!(out, "[{}] <未识别>", label_name(node.label));
                append_caption(caption, out);
            }
            NodeKind::Decoration { text } => {
                let name = label_name(node.label);
                match text {
                    Some(t) => {
                        let _ = writeln!(out, "[{name}] {t}");
                    }
                    None => {
                        let _ = writeln!(out, "[{name}] <跳过>");
                    }
                }
            }
        }
    }
}

fn append_caption(caption: &Option<String>, out: &mut String) {
    if let Some(c) = caption {
        if !c.is_empty() {
            let _ = writeln!(out, "    图注: {c}");
        }
    }
}

fn label_name(label: usize) -> &'static str {
    LABELS.get(label).copied().unwrap_or("unknown")
}

fn line(out: &mut String, label: usize, text: &str) {
    let _ = writeln!(out, "[{}] {text}", label_name(label));
}
