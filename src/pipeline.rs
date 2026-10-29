//! 端到端流水线：图片 → 版面 → 识别 → 文档树。
//!
//! CLI 与 HTTP 服务共用这一层，模型只在这里加载一次。

use burn::tensor::Device;
use image::RgbImage;

use crate::document::{Block, Document, ImageData, Item};
use crate::layout::{LayoutDetector, Region};
use crate::ocr::Recognizer;
use crate::table;

pub struct Pipeline {
    layout: LayoutDetector,
    recognizer: Recognizer,
}

impl Pipeline {
    pub fn new() -> Self {
        let device = Device::default();
        Self {
            layout: LayoutDetector::new(&device),
            recognizer: Recognizer::new(&device),
        }
    }

    /// `use_layout = false` 时跳过版面模型，整页当成一块文本。
    pub fn analyze(&self, rgb: &RgbImage, use_layout: bool) -> Document {
        let regions = if use_layout {
            self.layout.detect(rgb)
        } else {
            vec![Region {
                label: 5,
                score: 1.0,
                x0: 0,
                y0: 0,
                x1: rgb.width() as usize,
                y1: rgb.height() as usize,
            }]
        };

        let items: Vec<Item> = regions
            .iter()
            .map(|r| {
                let block = match r.label {
                    1 | 15 | 16 => Block::Image(ImageData::from_rgb(&r.crop(rgb))),
                    8 => Block::Table(table::recognize(&r.crop(rgb), &self.recognizer)),
                    _ if r.is_text() => Block::Text(self.recognizer.recognize(&r.crop(rgb))),
                    _ => Block::Empty,
                };
                Item {
                    region: *r,
                    block,
                }
            })
            .collect();

        Document::build(rgb.width() as usize, rgb.height() as usize, items)
    }
}
