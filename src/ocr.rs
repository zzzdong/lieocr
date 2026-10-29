//! 文本行识别（PP-OCRv6_medium_rec）：区域图 → 文字。

use burn::tensor::{Device, Tensor, TensorData};
use image::RgbImage;

use crate::model::paddleocr::Model;
use crate::preprocess::Preprocess;

/// 识别模型输入高度固定 48（ONNX 输入 [N, 3, 48, W]，W 动态）。
const IMG_H: u32 = 48;
/// 输入宽度上限（模型导出的动态 shape 上限为 3200，这里取折中值避免长行被过度压缩）。
const MAX_W: u32 = 1280;
/// 字符字典：每行一个字符，顺序即模型输出的类别顺序（0 为 CTC blank）。
const DICT: &str = include_str!("../models/ppocr_keys.txt");

pub struct Recognizer {
    device: Device,
    model: Model,
    vocab: Vocab,
}

impl Recognizer {
    pub fn new(device: &Device) -> Self {
        Self {
            device: device.clone(),
            model: Model::from_file(crate::model::weight_path("paddleocr.bpk"), device),
            vocab: Vocab::new(),
        }
    }

    /// 对一块区域做「行检测 + 逐行识别」，行间直接拼接成一段文字。
    pub fn recognize(&self, crop: &RgbImage) -> String {
        let prep = Preprocess::new(crop);
        let lines = prep.detect_lines();
        let debug = std::env::var("LIEOCR_DEBUG").is_ok();
        if debug {
            println!(
                "    区域 {}x{}，切出 {} 行",
                crop.width(),
                crop.height(),
                lines.len()
            );
        }

        let mut out = String::new();
        for rect in &lines {
            let (pixels, cw, ch) = prep.crop(rect);
            let (data, wid) = to_model_input(&pixels, cw, ch);
            let input = Tensor::<4>::from_data(
                TensorData::new(data, [1, 3, IMG_H as usize, wid as usize]),
                &self.device,
            );
            let (line, score) = self.vocab.decode(self.model.forward(input));
            if debug {
                println!("      ({cw:>4}x{ch:<3}) conf={score:.3} {line}");
            }
            // 折行处两侧都是西文字符时补空格，中文直接相连
            if let (Some(prev), Some(next)) = (out.chars().last(), line.chars().next()) {
                if prev.is_ascii_alphanumeric() && next.is_ascii_alphanumeric() {
                    out.push(' ');
                }
            }
            out.push_str(&line);
        }
        out
    }
}

/// 等比缩放到高 48，按官方 PreProcess 做 (x/255 - 0.5) / 0.5 归一化（灰度三通道）。
fn to_model_input(pixels: &[u8], cw: usize, ch: usize) -> (Vec<f32>, u32) {
    let wid = ((IMG_H as f32 * cw as f32 / ch as f32).ceil() as u32).clamp(8, MAX_W);
    let plane = IMG_H as usize * wid as usize;
    let mut data = vec![0f32; 3 * plane];

    let scale_x = cw as f32 / wid as f32;
    let scale_y = ch as f32 / IMG_H as f32;
    for y in 0..IMG_H as usize {
        let sy = ((y as f32 + 0.5) * scale_y - 0.5).clamp(0.0, ch as f32 - 1.0);
        let (y0, fy) = (sy.floor() as usize, sy.fract());
        let y1 = (y0 + 1).min(ch - 1);
        for x in 0..wid as usize {
            let sx = ((x as f32 + 0.5) * scale_x - 0.5).clamp(0.0, cw as f32 - 1.0);
            let (x0, fx) = (sx.floor() as usize, sx.fract());
            let x1 = (x0 + 1).min(cw - 1);

            let p00 = pixels[y0 * cw + x0] as f32;
            let p01 = pixels[y0 * cw + x1] as f32;
            let p10 = pixels[y1 * cw + x0] as f32;
            let p11 = pixels[y1 * cw + x1] as f32;
            let top = p00 * (1.0 - fx) + p01 * fx;
            let bottom = p10 * (1.0 - fx) + p11 * fx;
            let v = norm((top * (1.0 - fy) + bottom * fy).round() as u8);

            let idx = y * wid as usize + x;
            data[idx] = v;
            data[plane + idx] = v;
            data[2 * plane + idx] = v;
        }
    }
    (data, wid)
}

fn norm(v: u8) -> f32 {
    (v as f32 / 255.0 - 0.5) / 0.5
}

struct Vocab {
    chars: Vec<char>,
}

impl Vocab {
    fn new() -> Self {
        // 与 PaddleOCR 一致：blank 占 0，字典字符从 1 开始，末尾追加空格。
        let mut chars: Vec<char> = DICT.lines().flat_map(|line| line.chars()).collect();
        chars.push(' ');
        Self { chars }
    }

    fn get(&self, idx: usize) -> Option<char> {
        if idx == 0 {
            None
        } else {
            self.chars.get(idx - 1).copied()
        }
    }

    /// CTC 贪心解码：合并连续重复、去掉 blank。
    fn decode(&self, logits: Tensor<3>) -> (String, f32) {
        let dims = logits.dims();
        let (t, c) = (dims[1], dims[2]);
        let data = logits
            .into_data()
            .try_to_vec::<f32>()
            .expect("无法读取模型输出张量");

        let mut out = String::new();
        let (mut conf_sum, mut count, mut prev) = (0f32, 0usize, usize::MAX);
        for step in 0..t {
            let row = &data[step * c..(step + 1) * c];
            let mut best = 0usize;
            let mut best_v = f32::NEG_INFINITY;
            for (idx, &v) in row.iter().enumerate() {
                if v > best_v {
                    best = idx;
                    best_v = v;
                }
            }
            if best != 0 && best != prev {
                if let Some(ch) = self.get(best) {
                    out.push(ch);
                    conf_sum += best_v;
                    count += 1;
                }
            }
            prev = best;
        }

        let score = if count > 0 { conf_sum / count as f32 } else { 0.0 };
        (out, score)
    }
}
