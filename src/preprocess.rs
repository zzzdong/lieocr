//! 图像预处理与文本行定位。
//!
//! 流程：
//! 1. 灰度化 → 大窗口均值估计背景 → 加法式去阴影（手机拍摄、纸面光照不均的关键）；
//! 2. 自适应阈值（局部均值 - C）二值化；
//! 3. 去掉细长横线（笔记本横线、表格线）与细短噪点；
//! 4. 3x3 膨胀连回断笔，水平投影成带；
//! 5. 过高的带（多行粘在一起）按投影谷值递归切开——手写笔画上下出格时，行间不存在
//!    「整行全空」的空隙，所以必须靠谷值而不是空行来切。

use image::RgbImage;

/// 行区域，`x1/y1` 为开区间。
#[derive(Clone, Copy, Debug)]
pub struct LineBox {
    pub x0: usize,
    pub y0: usize,
    pub x1: usize,
    pub y1: usize,
}

impl LineBox {
    pub fn width(&self) -> usize {
        self.x1 - self.x0
    }

    pub fn height(&self) -> usize {
        self.y1 - self.y0
    }
}

/// 前景连通域（纵向范围 + 像素数），用于估计单字高度与判断带里有没有「字」。
#[derive(Clone, Copy, Debug)]
struct Comp {
    y0: usize,
    y1: usize,
    area: usize,
}

impl Comp {
    fn height(&self) -> usize {
        self.y1 - self.y0 + 1
    }
}

pub struct Preprocess {
    w: usize,
    h: usize,
    /// 去阴影后的灰度图（送给识别模型）
    flat: Vec<u8>,
    /// 前景 mask（已去横线、已膨胀）
    mask: Vec<bool>,
    /// mask 的连通域，用于区分「文字」与「线」
    comps: Vec<Comp>,
    /// 单字高度估计，用于判断一个投影带里是否不止一行
    line_h: usize,
}

impl Preprocess {
    pub fn new(img: &RgbImage) -> Self {
        let (flat, w, h) = flatten_shadow(img);
        let mut mask = foreground_mask(&flat, w, h);

        remove_thin_lines(&mut mask, w, h);
        let mask = dilate(&mask, w, h);
        let (line_h, comps) = analyze_components(&mask, w, h);

        if std::env::var("LIEOCR_DEBUG").is_ok() {
            let dump: Vec<u8> = mask.iter().map(|&v| if v { 0 } else { 255 }).collect();
            let _ = image::GrayImage::from_raw(w as u32, h as u32, dump)
                .expect("mask size")
                .save("target/debug_mask.png");
            let _ = image::GrayImage::from_raw(w as u32, h as u32, flat.clone())
                .expect("flat size")
                .save("target/debug_flat.png");
            println!("line_h = {line_h}, comps = {}", comps.len());
        }

        Self { w, h, flat, mask, comps, line_h }
    }

    /// 定位文本行，按从上到下排序。
    pub fn detect_lines(&self) -> Vec<LineBox> {
        let (w, h) = (self.w, self.h);

        let profile: Vec<u32> = (0..h)
            .map(|y| {
                let row = &self.mask[y * w..(y + 1) * w];
                row.iter().filter(|&&v| v).count() as u32
            })
            .collect();

        // 先按「有墨迹」成带，再把过高的带切开
        let mut bands: Vec<(usize, usize)> = Vec::new();
        let mut y = 0;
        while y < h {
            if profile[y] == 0 {
                y += 1;
                continue;
            }
            let start = y;
            while y + 1 < h && profile[y + 1] > 0 {
                y += 1;
            }
            bands.push((start, y));
            y += 1;
        }

        let mut lines = Vec::new();
        for (y0, y1) in bands {
            split_band(&profile, y0, y1, self.line_h, &mut lines);
        }

        let pad = 2;
        let mut boxes: Vec<LineBox> = Vec::new();
        for (y0, y1) in lines {
            let mut x0 = w;
            let mut x1 = 0;
            for yy in y0..=y1 {
                let row = &self.mask[yy * w..(yy + 1) * w];
                for (x, &v) in row.iter().enumerate() {
                    if v {
                        x0 = x0.min(x);
                        x1 = x1.max(x);
                    }
                }
            }
            if x1 <= x0 || y1 - y0 + 1 < 6 || x1 - x0 + 1 < 10 {
                continue;
            }
            // 带里必须存在「接近字高」的连通域才算文字行；横线残片只有几像素高
            let has_glyph = self
                .comps
                .iter()
                .any(|c| c.y0 >= y0.saturating_sub(1) && c.y1 <= y1 + 1 && c.height() * 2 >= self.line_h);
            if !has_glyph {
                continue;
            }
            boxes.push(LineBox {
                x0: x0.saturating_sub(pad),
                y0: y0.saturating_sub(pad),
                x1: (x1 + 1 + pad).min(w),
                y1: (y1 + 1 + pad).min(h),
            });
        }
        boxes.sort_by_key(|b| (b.y0, b.x0));
        boxes
    }

    /// 取一行的灰度像素（行宽、行高）。
    pub fn crop(&self, b: &LineBox) -> (Vec<u8>, usize, usize) {
        let (cw, ch) = (b.width(), b.height());
        let mut out = Vec::with_capacity(cw * ch);
        for y in b.y0..b.y1 {
            let start = y * self.w + b.x0;
            out.extend_from_slice(&self.flat[start..start + cw]);
        }
        (out, cw, ch)
    }
}

/// 灰度化 + 加法式去阴影（白纸保持 255，阴影被抬亮，文字灰度基本不变）。
pub(crate) fn flatten_shadow(img: &RgbImage) -> (Vec<u8>, usize, usize) {
    let (w, h) = (img.width() as usize, img.height() as usize);
    let gray: Vec<u8> = img
        .pixels()
        .map(|p| (0.299 * p[0] as f32 + 0.587 * p[1] as f32 + 0.114 * p[2] as f32).round() as u8)
        .collect();

    // 背景估计窗口要远大于笔画宽度、又小于光照变化尺度
    let radius = (w.min(h) / 15).clamp(8, 96);
    let bg = box_blur(&gray, w, h, radius);
    let flat = gray
        .iter()
        .zip(bg.iter())
        .map(|(&g, &b)| (g as i32 + 255 - b as i32).clamp(0, 255) as u8)
        .collect();
    (flat, w, h)
}

/// 自适应二值化：明显暗于局部背景才算墨迹（未去线、未膨胀）。
pub(crate) fn foreground_mask(flat: &[u8], w: usize, h: usize) -> Vec<bool> {
    let radius = (w.min(h) / 15).clamp(8, 96);
    let bg = box_blur(flat, w, h, radius);
    flat.iter()
        .zip(bg.iter())
        .map(|(&v, &b)| (v as i32) < (b as i32 - 10))
        .collect()
}

/// 把一个投影带切成若干行：过高就找最深的谷递归切分。
fn split_band(profile: &[u32], y0: usize, y1: usize, line_h: usize, out: &mut Vec<(usize, usize)>) {
    let height = y1 - y0 + 1;
    if height <= line_h * 17 / 10 {
        out.push((y0, y1));
        return;
    }

    // 端点附近往往是笔画出格处，不当作切分点
    let margin = (height / 8).max(2);
    let (a, b) = (y0 + margin, y1 - margin);
    if a >= b {
        out.push((y0, y1));
        return;
    }

    let mut best = a;
    for y in a..=b {
        if profile[y] < profile[best] {
            best = y;
        }
    }
    let peak = (y0..=y1).map(|y| profile[y]).max().unwrap_or(1);
    // 谷要足够深才算两行之间的分界
    if profile[best] * 4 >= peak * 3 {
        out.push((y0, y1));
        return;
    }
    split_band(profile, y0, best, line_h, out);
    split_band(profile, best + 1, y1, line_h, out);
}

/// 提取连通域，并用「像字」的连通域高度中位数估计单字高度。
fn analyze_components(mask: &[bool], w: usize, h: usize) -> (usize, Vec<Comp>) {
    let mut comps: Vec<Comp> = Vec::new();
    let mut seen = vec![false; mask.len()];
    let mut stack: Vec<usize> = Vec::new();

    for seed in 0..mask.len() {
        if !mask[seed] || seen[seed] {
            continue;
        }
        let mut area = 0usize;
        let (mut y0, mut y1) = (h, 0usize);
        seen[seed] = true;
        stack.push(seed);
        while let Some(p) = stack.pop() {
            let (x, y) = (p % w, p / w);
            area += 1;
            y0 = y0.min(y);
            y1 = y1.max(y);
            let (ys, ye) = (y.saturating_sub(1), (y + 2).min(h));
            let (xs, xe) = (x.saturating_sub(1), (x + 2).min(w));
            for yy in ys..ye {
                for xx in xs..xe {
                    let q = yy * w + xx;
                    if mask[q] && !seen[q] {
                        seen[q] = true;
                        stack.push(q);
                    }
                }
            }
        }
        comps.push(Comp { y0, y1, area });
    }

    // 太矮的连通域多是横线碎片/噪点，会拉低估计
    let mut heights: Vec<usize> = comps
        .iter()
        .filter(|c| c.area >= 20 && c.height() >= 8)
        .map(|c| c.height())
        .collect();
    let line_h = if heights.is_empty() {
        16
    } else {
        heights.sort_unstable();
        heights[heights.len() / 2].max(6)
    };
    (line_h, comps)
}

/// 去掉细长横线：垂直厚度 <= 3 且水平延伸足够长的像素段（笔记本横线、表格线）。
fn remove_thin_lines(mask: &mut [bool], w: usize, h: usize) {
    // mask 只用于定位（识别用的是去阴影原图），所以这里可以激进一点
    let max_thickness = (w.min(h) / 160).max(3);
    let mut thin = vec![false; w * h];
    for x in 0..w {
        let mut y = 0;
        while y < h {
            if !mask[y * w + x] {
                y += 1;
                continue;
            }
            let start = y;
            while y < h && mask[y * w + x] {
                y += 1;
            }
            if y - start <= max_thickness {
                for yy in start..y {
                    thin[yy * w + x] = true;
                }
            }
        }
    }

    let min_len = (w / 4).max(80);
    // 倾斜的横线在水平方向是断续的，先按「整行薄像素占比」整行删掉
    for y in 0..h {
        let count = (0..w).filter(|&x| thin[y * w + x]).count();
        if count * 100 >= w * 35 {
            for x in 0..w {
                if thin[y * w + x] {
                    mask[y * w + x] = false;
                }
            }
        }
    }
    for y in 0..h {
        let mut x = 0;
        while x < w {
            if !thin[y * w + x] {
                x += 1;
                continue;
            }
            let start = x;
            let mut end = x;
            let mut gap = 0;
            while x < w {
                if thin[y * w + x] {
                    gap = 0;
                    end = x;
                    x += 1;
                } else {
                    gap += 1;
                    // 倾斜的横线被抗锯齿/阈值切成很多段，需要容忍较大间隙
                    if gap > 8 {
                        break;
                    }
                    x += 1;
                }
            }
            if end - start + 1 >= min_len || (end - start + 1) * 4 >= w * 3 {
                for xx in start..=end {
                    mask[y * w + xx] = false;
                }
            }
        }
    }
}

fn box_blur(src: &[u8], w: usize, h: usize, r: usize) -> Vec<u8> {
    // 积分图，O(1) 窗口求和
    let mut integral = vec![0u64; (w + 1) * (h + 1)];
    for y in 0..h {
        let mut row_sum = 0u64;
        for x in 0..w {
            row_sum += src[y * w + x] as u64;
            integral[(y + 1) * (w + 1) + x + 1] = integral[y * (w + 1) + x + 1] + row_sum;
        }
    }
    let mut out = vec![0u8; w * h];
    for y in 0..h {
        let y0 = y.saturating_sub(r);
        let y1 = (y + r + 1).min(h);
        for x in 0..w {
            let x0 = x.saturating_sub(r);
            let x1 = (x + r + 1).min(w);
            let sum = integral[y1 * (w + 1) + x1] - integral[y0 * (w + 1) + x1]
                - integral[y1 * (w + 1) + x0]
                + integral[y0 * (w + 1) + x0];
            out[y * w + x] = (sum / ((y1 - y0) * (x1 - x0)) as u64) as u8;
        }
    }
    out
}

fn dilate(mask: &[bool], w: usize, h: usize) -> Vec<bool> {
    let mut out = mask.to_vec();
    for y in 0..h {
        for x in 0..w {
            if !mask[y * w + x] {
                continue;
            }
            let (ys, ye) = (y.saturating_sub(1), (y + 2).min(h));
            let (xs, xe) = (x.saturating_sub(1), (x + 2).min(w));
            for yy in ys..ye {
                for xx in xs..xe {
                    out[yy * w + xx] = true;
                }
            }
        }
    }
    out
}
