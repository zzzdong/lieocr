//! 版面区域检测（PP-DocLayout_plus-L，RT-DETR）。
//!
//! 模型接口（见 `model/inference.layout.yml`）：
//! - 输入：`image [N,3,800,800]`（直接拉伸到 800x800、BGR、保持 0-255 不归一化）、
//!   `im_shape [N,2] = [[800,800]]`、`scale_factor [N,2] = [[800/h, 800/w]]`
//! - 输出：`[300*N, 6]`，每行 `[类别, 置信度, x1, y1, x2, y2]`（800x800 坐标系）

use burn::tensor::{Device, Tensor, TensorData};
use image::imageops::FilterType;
use image::RgbImage;

use crate::model::layout::Model;

/// 版面类别，顺序与 `inference.layout.yml` 的 `label_list` 一致。
pub const LABELS: [&str; 20] = [
    "paragraph_title",
    "image",
    "text",
    "number",
    "abstract",
    "content",
    "figure_title",
    "formula",
    "table",
    "reference",
    "doc_title",
    "footnote",
    "header",
    "algorithm",
    "footer",
    "seal",
    "chart",
    "formula_number",
    "aside_text",
    "reference_content",
];

const INPUT_SIZE: u32 = 800;
const SCORE_THRESHOLD: f32 = 0.5;

#[derive(Clone, Copy, Debug)]
pub struct Region {
    pub label: usize,
    pub score: f32,
    /// 原图坐标（x1/y1 为开区间）
    pub x0: usize,
    pub y0: usize,
    pub x1: usize,
    pub y1: usize,
}

impl Region {
    #[allow(dead_code)]
    pub fn name(&self) -> &'static str {
        LABELS.get(self.label).copied().unwrap_or("unknown")
    }

    /// 该区域里应当做文字识别（而不是跳过的图/表/公式）。
    pub fn is_text(&self) -> bool {
        matches!(
            self.label,
            0 | 2 | 4 | 5 | 6 | 9 | 10 | 11 | 12 | 13 | 14 | 18 | 19
        )
    }

    /// 从原图裁出该区域。
    pub fn crop(&self, img: &RgbImage) -> RgbImage {
        let (cw, ch) = (self.x1 - self.x0, self.y1 - self.y0);
        let mut out = RgbImage::new(cw as u32, ch as u32);
        for y in 0..ch {
            for x in 0..cw {
                out.put_pixel(
                    x as u32,
                    y as u32,
                    *img.get_pixel((self.x0 + x) as u32, (self.y0 + y) as u32),
                );
            }
        }
        out
    }
}

pub struct LayoutDetector {
    model: Model,
    device: Device,
}

impl LayoutDetector {
    pub fn new(device: &Device) -> Self {
        Self {
            model: Model::from_file(crate::model::weight_path("layout.bpk"), device),
            device: device.clone(),
        }
    }

    pub fn detect(&self, img: &RgbImage) -> Vec<Region> {
        let (w, h) = (img.width(), img.height());

        // 官方 yml 是 keep_ratio:false（直接拉伸），但对宽扁截图会把文字拉得过高，
        // 用 LIEOCR_LB=1 可切换成保持比例的 letterbox。
        let letterbox = std::env::var("LIEOCR_LB").is_ok();
        let (resized, sf) = if letterbox {
            let s = (INPUT_SIZE as f32 / w as f32).min(INPUT_SIZE as f32 / h as f32);
            let nw = ((w as f32 * s).round() as u32).clamp(1, INPUT_SIZE);
            let nh = ((h as f32 * s).round() as u32).clamp(1, INPUT_SIZE);
            let small = image::imageops::resize(img, nw, nh, FilterType::CatmullRom);
            let mut canvas =
                RgbImage::from_pixel(INPUT_SIZE, INPUT_SIZE, image::Rgb([114, 114, 114]));
            image::imageops::replace(&mut canvas, &small, 0, 0);
            (canvas, [s, s])
        } else {
            (
                image::imageops::resize(img, INPUT_SIZE, INPUT_SIZE, FilterType::CatmullRom),
                [
                    INPUT_SIZE as f32 / h.max(1) as f32,
                    INPUT_SIZE as f32 / w.max(1) as f32,
                ],
            )
        };

        // 预处理组合先用环境变量试：bgr255 / rgb255 / bgr1 / rgb1 / rgbnet
        let mode = std::env::var("LIEOCR_PREP").unwrap_or_else(|_| "bgr1".to_owned());
        let (order, scale, net) = match mode.as_str() {
            "rgb255" => ([2usize, 1, 0], 1.0f32, None),
            "bgr1" => ([0usize, 1, 2], 1.0 / 255.0, None),
            "rgb1" => ([2usize, 1, 0], 1.0 / 255.0, None),
            "rgbnet" => (
                [2usize, 1, 0],
                1.0 / 255.0,
                Some(([0.485f32, 0.456, 0.406], [0.229f32, 0.224, 0.225])),
            ),
            _ => ([0usize, 1, 2], 1.0f32, None),
        };

        let plane = (INPUT_SIZE * INPUT_SIZE) as usize;
        let mut data = vec![0f32; 3 * plane];
        for (i, p) in resized.pixels().enumerate() {
            for c in 0..3 {
                let mut v = p[order[c]] as f32 * scale;
                if let Some((mean, std)) = net {
                    v = (v - mean[c]) / std[c];
                }
                data[c * plane + i] = v;
            }
        }

        let image = Tensor::<4>::from_data(
            TensorData::new(data, [1, 3, INPUT_SIZE as usize, INPUT_SIZE as usize]),
            &self.device,
        );
        // im_shape 是网络输入尺寸（不是原图尺寸），模型据此把框映射回原图
        let im_shape = Tensor::<2>::from_data(
            TensorData::new(vec![INPUT_SIZE as f32, INPUT_SIZE as f32], [1, 2]),
            &self.device,
        );
        let scale_factor =
            Tensor::<2>::from_data(TensorData::new(vec![sf[0], sf[1]], [1, 2]), &self.device);

        let (det, _) = self.model.forward(im_shape, image, scale_factor);
        let dims = det.dims();
        let data = det
            .into_data()
            .try_to_vec::<f32>()
            .expect("无法读取版面模型输出");

        let stride = dims[1];
        if std::env::var("LIEOCR_DEBUG").is_ok() {
            let mut idx: Vec<usize> = (0..dims[0]).collect();
            idx.sort_by(|&a, &b| {
                data[b * stride + 1]
                    .partial_cmp(&data[a * stride + 1])
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            println!("layout prep={mode} im={w}x{h} dims={dims:?}");
            for &r in idx.iter().take(8) {
                let s = r * stride;
                println!("  raw = {:?}", &data[s..s + stride]);
            }
        }
        let thr = std::env::var("LIEOCR_SCORE")
            .ok()
            .and_then(|v| v.parse::<f32>().ok())
            .unwrap_or(SCORE_THRESHOLD);

        let mut regions = Vec::new();
        for row in 0..dims[0] {
            let r = &data[row * stride..(row + 1) * stride];
            let (label, score) = (r[0], r[1]);
            if score < thr {
                continue;
            }
            // 模型已按 scale_factor 把框映射回原图尺度，这里只需裁剪到图内
            let (x1, y1, x2, y2) = (r[2], r[3], r[4], r[5]);
            let x0 = x1.clamp(0.0, w as f32) as usize;
            let yy0 = y1.clamp(0.0, h as f32) as usize;
            let x1 = x2.clamp(0.0, w as f32) as usize;
            let yy1 = y2.clamp(0.0, h as f32) as usize;
            if x1 <= x0 || yy1 - yy0 < 8 {
                continue;
            }
            regions.push(Region {
                label: label.round().max(0.0) as usize,
                score,
                x0,
                y0: yy0,
                x1,
                y1: yy1,
            });
        }

        reading_order(dedup(regions))
    }
}

/// DETR 是 NMS-free 的，同一块内容可能被不同类别重复框出（如 text + content），
/// 这里按重合度去重，保留置信度更高的那个。
fn dedup(mut regions: Vec<Region>) -> Vec<Region> {
    regions.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut kept: Vec<Region> = Vec::with_capacity(regions.len());
    for r in regions {
        if kept.iter().any(|k| iou(k, &r) > 0.75) {
            continue;
        }
        kept.push(r);
    }
    kept
}

fn iou(a: &Region, b: &Region) -> f32 {
    let x0 = a.x0.max(b.x0);
    let y0 = a.y0.max(b.y0);
    let x1 = a.x1.min(b.x1);
    let y1 = a.y1.min(b.y1);
    if x1 <= x0 || y1 <= y0 {
        return 0.0;
    }
    let inter = (x1 - x0) * (y1 - y0);
    let area_a = (a.x1 - a.x0) * (a.y1 - a.y0);
    let area_b = (b.x1 - b.x0) * (b.y1 - b.y0);
    inter as f32 / (area_a + area_b - inter) as f32
}

/// 阅读顺序：先按纵向聚成行，行内按横向排序（多栏文档常见的一层 XY-cut）。
fn reading_order(mut regions: Vec<Region>) -> Vec<Region> {
    regions.sort_by(|a, b| a.y0.cmp(&b.y0));

    let mut rows: Vec<Vec<Region>> = Vec::new();
    for r in regions {
        if let Some(row) = rows.last_mut() {
            let ry0 = row.iter().map(|x| x.y0).min().unwrap_or(0);
            let ry1 = row.iter().map(|x| x.y1).max().unwrap_or(0);
            let overlap = ry1.min(r.y1) as i64 - ry0.max(r.y0) as i64;
            let min_h = (ry1 - ry0).min(r.y1 - r.y0).max(1) as i64;
            if overlap * 2 > min_h {
                row.push(r);
                continue;
            }
        }
        rows.push(vec![r]);
    }
    for row in &mut rows {
        row.sort_by(|a, b| a.x0.cmp(&b.x0));
    }
    rows.into_iter().flatten().collect()
}
