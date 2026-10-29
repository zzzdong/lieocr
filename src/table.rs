//! 表格还原：网格线检测 + 单元格文字识别 → 行 × 列文本。
//!
//! 有线表格用「横线/竖线在整行/整列中的占比」定位单元格边界；
//! 没有明显框线时退化为「投影空白带」切分。合并单元格（rowspan/colspan）不做还原，
//! 因为 Markdown 表格本身不支持。

use image::RgbImage;

use crate::ocr::Recognizer;
use crate::preprocess::{flatten_shadow, foreground_mask};

/// 识别一张表格图，返回行 × 列的单元格文字。
pub fn recognize(img: &RgbImage, rec: &Recognizer) -> Vec<Vec<String>> {
    let (flat, w, h) = flatten_shadow(img);
    if w < 16 || h < 16 {
        return Vec::new();
    }
    let mask = foreground_mask(&flat, w, h);

    // 优先用网格线；线不够就退化成投影
    let mut rows = line_boundaries(&mask, w, h, true);
    let mut cols = line_boundaries(&mask, w, h, false);
    if rows.len() < 2 || cols.len() < 2 {
        rows = projection_boundaries(&mask, w, h, true);
        cols = projection_boundaries(&mask, w, h, false);
    }
    if rows.len() < 2 || cols.len() < 2 {
        return Vec::new();
    }

    // 内缩，避开门框线本身
    let pad = 3;
    let mut out = Vec::new();
    for pair in rows.windows(2) {
        let y0 = pair[0] + pad;
        let y1 = pair[1].saturating_sub(pad);
        if y1 <= y0 {
            continue;
        }
        let mut row = Vec::new();
        for cp in cols.windows(2) {
            let x0 = cp[0] + pad;
            let x1 = cp[1].saturating_sub(pad);
            if x1 <= x0 {
                row.push(String::new());
                continue;
            }
            let cell = crop(img, x0, y0, x1 - x0, y1 - y0);
            row.push(rec.recognize(&cell).trim().to_owned());
        }
        out.push(row);
    }
    out
}

/// 通过「细长且贯穿」的暗像素比例找横线/竖线的中心位置。
fn line_boundaries(mask: &[bool], w: usize, h: usize, horizontal: bool) -> Vec<usize> {
    let n = if horizontal { h } else { w };
    let m = if horizontal { w } else { h };

    let mut flags = vec![false; n];
    for i in 0..n {
        let mut cnt = 0usize;
        for j in 0..m {
            let idx = if horizontal { i * w + j } else { j * w + i };
            if mask[idx] {
                cnt += 1;
            }
        }
        // 贯穿 65% 以上才算框线（表格内文字行达不到）
        if cnt * 100 >= m * 65 {
            flags[i] = true;
        }
    }

    let mut bounds: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < n {
        if !flags[i] {
            i += 1;
            continue;
        }
        let start = i;
        while i + 1 < n && flags[i + 1] {
            i += 1;
        }
        let center = (start + i) / 2;
        // 双线表：间隔太近的合并
        if bounds.last().is_none_or(|&b| center - b > 6) {
            bounds.push(center);
        }
        i += 1;
    }

    if bounds.len() >= 2 {
        // 缺外框时补上边缘
        if bounds[0] > 6 {
            bounds.insert(0, 2);
        }
        if *bounds.last().unwrap() + 6 < n {
            bounds.push(n - 2);
        }
    }
    bounds
}

/// 没有框线时的退化方案：用整行/整列墨迹量找空白带作为分隔。
fn projection_boundaries(mask: &[bool], w: usize, h: usize, horizontal: bool) -> Vec<usize> {
    let n = if horizontal { h } else { w };
    let m = if horizontal { w } else { h };

    let mut ink = vec![0usize; n];
    for i in 0..n {
        let mut cnt = 0usize;
        for j in 0..m {
            let idx = if horizontal { i * w + j } else { j * w + i };
            if mask[idx] {
                cnt += 1;
            }
        }
        ink[i] = cnt;
    }

    let max = ink.iter().copied().max().unwrap_or(0);
    if max == 0 {
        return Vec::new();
    }
    // 低于峰值 1% 视为空白
    let thr = (max / 100).max(1);

    let mut bounds = vec![0usize];
    let mut i = 0;
    while i < n {
        if ink[i] > thr {
            i += 1;
            continue;
        }
        let start = i;
        while i + 1 < n && ink[i + 1] <= thr {
            i += 1;
        }
        let center = (start + i + 1) / 2;
        if center > 8
            && center + 8 < n
            && bounds.last().is_none_or(|&b| center - b > 10)
        {
            bounds.push(center);
        }
        i += 1;
    }
    if bounds.last().is_some_and(|&b| b + 8 < n) {
        bounds.push(n);
    }
    if bounds.len() < 2 {
        Vec::new()
    } else {
        bounds
    }
}

fn crop(src: &RgbImage, x0: usize, y0: usize, cw: usize, ch: usize) -> RgbImage {
    let mut out = RgbImage::new(cw as u32, ch as u32);
    for y in 0..ch {
        for x in 0..cw {
            out.put_pixel(
                x as u32,
                y as u32,
                *src.get_pixel((x0 + x) as u32, (y0 + y) as u32),
            );
        }
    }
    out
}
