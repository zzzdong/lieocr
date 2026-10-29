//! 模型层：`models/` 目录统一存放 ONNX 源文件、onnx2burn 生成的 `.rs` / `.bpk`
//! 以及字符字典。

#[path = "../models/layout.rs"]
pub mod layout;
#[path = "../models/paddleocr.rs"]
pub mod paddleocr;

use std::path::PathBuf;
use std::sync::OnceLock;

/// 由 `--models-dir` 或 `LIEOCR_MODEL_DIR` 指定的目录（进程内只设置一次）。
static MODELS_DIR: OnceLock<PathBuf> = OnceLock::new();

/// 设置模型目录：优先取命令行参数，其次环境变量。
pub fn set_models_dir(dir: Option<PathBuf>) {
    let dir = dir.or_else(|| std::env::var_os("LIEOCR_MODEL_DIR").map(PathBuf::from));
    if let Some(d) = dir {
        let _ = MODELS_DIR.set(d);
    }
}

/// 定位模型权重（`.bpk`）。
///
/// 查找顺序：
/// 1. `--models-dir` / `LIEOCR_MODEL_DIR` 指定的目录（找不到直接报错，避免静默用错模型）
/// 2. 可执行文件同级的 `models/`（发布包布局）
/// 3. 可执行文件上两级的 `models/`（`target/release/lieocr.exe` 的开发布局）
/// 4. 当前工作目录下的 `models/`
pub fn weight_path(name: &str) -> PathBuf {
    if let Some(dir) = MODELS_DIR.get() {
        let path = dir.join(name);
        assert!(
            path.is_file(),
            "模型目录 {} 中找不到 {name}",
            dir.display()
        );
        return path;
    }

    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join("models").join(name));
            candidates.push(dir.join("..").join("..").join("models").join(name));
        }
    }
    candidates.push(PathBuf::from("models").join(name));

    for c in &candidates {
        if c.is_file() {
            return c.clone();
        }
    }

    let tried = candidates
        .iter()
        .map(|p| format!("  {}", p.display()))
        .collect::<Vec<_>>()
        .join("\n");
    panic!(
        "找不到模型权重 {name}。请先运行 scripts/fetch-models.sh 生成，或用 --models-dir / LIEOCR_MODEL_DIR 指定目录。\n已尝试：\n{tried}"
    );
}
