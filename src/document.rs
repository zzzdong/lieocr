//! 版面结构化表示层。
//!
//! 版面检测只给出「扁平区域 + 阅读顺序」，识别只给出「区域里的文字」。
//! 这一层把它们合成一棵文档树：标题形成层级，正文/图/表/公式成为叶子节点，
//! 同时保留 bbox 与原始类别，供不同渲染后端（Markdown / 未来的 PDF、DOCX）使用。

use crate::layout::Region;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rect {
    pub x0: usize,
    pub y0: usize,
    pub x1: usize,
    pub y1: usize,
}

impl Rect {
    pub fn from_region(r: &Region) -> Self {
        Self {
            x0: r.x0,
            y0: r.y0,
            x1: r.x1,
            y1: r.y1,
        }
    }

    pub fn width(&self) -> usize {
        self.x1.saturating_sub(self.x0)
    }

    pub fn height(&self) -> usize {
        self.y1.saturating_sub(self.y0)
    }
}

/// 图片区域的像素（供 Markdown 内嵌 base64 / 未来 PDF 嵌图）。
#[derive(Debug)]
pub struct ImageData {
    pub width: u32,
    pub height: u32,
    /// RGB8，逐行排列
    pub rgb: Vec<u8>,
}

impl ImageData {
    pub fn from_rgb(img: &image::RgbImage) -> Self {
        Self {
            width: img.width(),
            height: img.height(),
            rgb: img.as_raw().clone(),
        }
    }
}

/// 一块版面区域的识别结果。
#[derive(Debug)]
pub enum Block {
    Text(String),
    /// 图片区域的像素
    Image(ImageData),
    /// 表格：行 × 列 的单元格文字
    Table(Vec<Vec<String>>),
    /// 无需识别（表头公式等占位）或识别失败
    Empty,
}

pub struct Item {
    pub region: Region,
    pub block: Block,
}

/// 一块版面内容在文档树中的语义。
#[derive(Debug)]
pub enum NodeKind {
    /// 文档标题（通常整篇只有一个）
    Heading { level: u8, text: String },
    /// 分节：标题 + 其下的内容
    Section {
        level: u8,
        title: String,
        children: Vec<Node>,
    },
    Paragraph { text: String },
    Figure {
        caption: Option<String>,
        image: Option<ImageData>,
    },
    Table {
        caption: Option<String>,
        rows: Vec<Vec<String>>,
    },
    Formula { caption: Option<String> },
    /// 页眉、页脚、页码等，不属于正文
    Decoration { text: Option<String> },
}

#[derive(Debug)]
pub struct Node {
    pub bbox: Rect,
    /// 版面模型的类别索引（`layout::LABELS`）
    pub label: usize,
    pub score: f32,
    pub kind: NodeKind,
}

impl Node {
    pub fn children(&self) -> &[Node] {
        match &self.kind {
            NodeKind::Section { children, .. } => children,
            _ => &[],
        }
    }
}

#[derive(Debug)]
pub struct Document {
    /// 原图尺寸，作为版面画布尺寸保留给 PDF/DOCX 渲染使用
    #[allow(dead_code)]
    pub width: usize,
    #[allow(dead_code)]
    pub height: usize,
    pub children: Vec<Node>,
}

impl Document {
    /// 由「版面区域 + 识别结果」构建文档树，顺序即阅读顺序。
    pub fn build(width: usize, height: usize, items: Vec<Item>) -> Self {
        let mut root: Vec<Node> = Vec::new();
        let mut section: Option<Node> = None;

        for item in items {
            let region = item.region;
            let bbox = Rect::from_region(&region);
            let text = match &item.block {
                Block::Text(t) => t.clone(),
                _ => String::new(),
            };

            let kind = match region.label {
                // doc_title
                10 => NodeKind::Heading { level: 1, text },
                // paragraph_title：开启新的一节
                0 => {
                    flush(&mut section, &mut root);
                    section = Some(Node {
                        bbox,
                        label: region.label,
                        score: region.score,
                        kind: NodeKind::Section {
                            level: 2,
                            title: text,
                            children: Vec::new(),
                        },
                    });
                    continue;
                }
                // figure_title：贴到相邻的图/表上做图注，否则当成普通段落
                6 => {
                    if attach_caption(&mut section, &mut root, &text) {
                        continue;
                    }
                    NodeKind::Paragraph { text }
                }
                1 | 15 | 16 => {
                    let image = match item.block {
                        Block::Image(img) => Some(img),
                        _ => None,
                    };
                    NodeKind::Figure {
                        caption: None,
                        image,
                    }
                }
                8 => {
                    let rows = match item.block {
                        Block::Table(rows) => rows,
                        _ => Vec::new(),
                    };
                    NodeKind::Table {
                        caption: None,
                        rows,
                    }
                }
                7 => NodeKind::Formula { caption: None },
                3 | 12 | 14 => NodeKind::Decoration {
                    text: if text.is_empty() { None } else { Some(text) },
                },
                _ => NodeKind::Paragraph { text },
            };

            let node = Node {
                bbox,
                label: region.label,
                score: region.score,
                kind,
            };
            push(&mut section, &mut root, node);
        }

        flush(&mut section, &mut root);
        Self {
            width,
            height,
            children: root,
        }
    }

    /// 深度优先遍历所有节点（供统计/搜索类后端使用）。
    #[allow(dead_code)]
    pub fn walk(&self) -> Vec<&Node> {
        fn visit<'a>(nodes: &'a [Node], out: &mut Vec<&'a Node>) {
            for n in nodes {
                out.push(n);
                visit(n.children(), out);
            }
        }
        let mut out = Vec::new();
        visit(&self.children, &mut out);
        out
    }
}

fn push(scope: &mut Option<Node>, root: &mut Vec<Node>, node: Node) {
    match scope {
        Some(s) => {
            if let NodeKind::Section { children, .. } = &mut s.kind {
                children.push(node);
            }
        }
        None => root.push(node),
    }
}

fn flush(scope: &mut Option<Node>, root: &mut Vec<Node>) {
    if let Some(section) = scope.take() {
        root.push(section);
    }
}

fn last_mut<'a>(scope: &'a mut Option<Node>, root: &'a mut Vec<Node>) -> Option<&'a mut Node> {
    match scope {
        Some(s) => match &mut s.kind {
            NodeKind::Section { children, .. } => children.last_mut(),
            _ => None,
        },
        None => root.last_mut(),
    }
}

/// 把图注贴到同一层级里最近的那个图/表上（图注通常紧跟在图下方）。
fn attach_caption(scope: &mut Option<Node>, root: &mut Vec<Node>, text: &str) -> bool {
    if text.is_empty() {
        return false;
    }
    match last_mut(scope, root).map(|n| &mut n.kind) {
        Some(NodeKind::Figure { caption, .. }) | Some(NodeKind::Table { caption, .. }) => {
            if caption.is_none() {
                *caption = Some(text.to_owned());
                return true;
            }
            false
        }
        _ => false,
    }
}
