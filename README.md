# lieocr

[![release](https://github.com/zzzdong/lieocr/actions/workflows/release.yml/badge.svg)](https://github.com/zzzdong/lieocr/actions/workflows/release.yml)
[![license](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

纯 Rust 的文档 OCR：**版面分析 → 文本识别 → 文档结构树 → Markdown**。

模型用 [burn](https://burn.dev) 直接推理（ONNX 经 [onnx2burn](https://github.com/tracel-ai/burn-onnx) 转换为原生 Rust 代码），
运行时不需要 Python、不需要 ONNX Runtime、Linux 下是单个静态链接的可执行文件。

## 特性

- **版面分析**：PP-DocLayout_plus-L（RT-DETR）检测 20 类版面区域，按阅读顺序输出
- **文本识别**：PP-OCRv6_medium_rec 逐行识别，字符集 18710 类（中英 + 符号）
- **鲁棒预处理**：去阴影、自适应二值化、去横线（笔记本行线/表格线）、连通域与投影行切分，手写与手机拍照同样可用
- **结构化**：输出文档树（标题层级 / 段落 / 图表 / 公式 / 页眉页脚），而不是一堆无结构的文本行
- **Markdown 渲染**：标题、段落、GFM 表格；图片区域裁图后以 **base64 data URI 内嵌**，无需外部文件
- **两种入口**：命令行（单图识别）与 HTTP 服务（上传识别）
- **渲染层可扩展**：`Renderer` trait 预留 PDF / DOCX 后端

## 快速开始

### 使用发布包

从 [Releases](https://github.com/zzzdong/lieocr/releases) 下载对应平台的压缩包，解压即用（压缩包内已含模型）：

```bash
./lieocr test.png -m          # 输出 Markdown
./lieocr test.png             # 输出结构树 + 纯文本
```

支持的平台：

| 包 | 说明 |
|---|---|
| `lieocr-v<version>-windows-x64.zip` | Windows x64（MSVC） |
| `lieocr-v<version>-linux-x64-musl.tar.gz` | Linux x86_64，静态链接 |
| `lieocr-v<version>-linux-aarch64-musl.tar.gz` | Linux arm64，静态链接 |

### 从源码构建

模型权重不进仓库，需要先下载 ONNX 并转换：

```bash
git clone https://github.com/zzzdong/lieocr.git
cd lieocr

# 依赖：curl、sha256sum、onnx2burn（cargo install burn-onnx --version 0.22.0）
bash scripts/fetch-models.sh     # 下载约 200 MB 并生成 models/*.bpk
cargo build --release
./target/release/lieocr test.png -m
```

`scripts/fetch-models.sh` 会校验 SHA256，重复执行时自动跳过已下载的部分。

## 命令行

```bash
lieocr <图片>                      # 版面 + 结构树 + 纯文本
lieocr <图片> -m                   # 纯净 Markdown 输出
lieocr <图片> -m -o doc.md         # 写入文件
lieocr ocr <图片> -m               # 等价的子命令形式
lieocr <图片> --no-layout          # 跳过版面模型，整页当成一块文本
lieocr server -p 8080              # 启动 HTTP 服务
```

| 参数 | 说明 |
|---|---|
| `-m, --markdown` | 输出 Markdown（stdout 不再混入日志） |
| `-o, --output <FILE>` | 结果写入文件 |
| `--no-layout` | 跳过版面检测 |
| `--models-dir <DIR>` | 指定模型权重目录（优先级最高） |

## HTTP 服务

```bash
lieocr server --host 127.0.0.1 -p 8080
```

| 接口 | 说明 |
|---|---|
| `GET /health` | 返回 `ok` |
| `POST /ocr` | multipart 字段 `file`；query：`format=json\|markdown\|text`、`layout=true\|false` |

```bash
curl -F "file=@doc.png" "http://127.0.0.1:8080/ocr?format=markdown"
curl -F "file=@doc.png" "http://127.0.0.1:8080/ocr"     # JSON
```

JSON 响应：

```json
{
  "width": 820,
  "height": 620,
  "markdown": "...",
  "text": "...",
  "nodes": [
    {"kind": "table", "label": "table", "score": 0.955, "bbox": [78, 108, 700, 330]}
  ]
}
```

## 模型

| 模型 | 用途 | 来源 | 体积 |
|---|---|---|---|
| PP-OCRv6_medium_rec | 文本行识别 | `PaddlePaddle/PP-OCRv6_medium_rec_onnx`（ModelScope） | 76 MB |
| PP-DocLayout_plus-L | 版面区域检测 | `PaddlePaddle/PP-DocLayout_plus-L_onnx`（ModelScope） | 132 MB |

模型权重（`.onnx` / `.bpk`）不进入 Git，按需重建；[onnx2burn](https://github.com/tracel-ai/burn-onnx) 生成的 `.rs`
与字符字典 `models/ppocr_keys.txt` 随仓库分发，因此**编译**不需要模型。

权重查找顺序：

1. `--models-dir` 指定的目录
2. 环境变量 `LIEOCR_MODEL_DIR`
3. 可执行文件同级的 `models/`（发布包布局）
4. 可执行文件上两级的 `models/`（`target/release/` 开发布局）
5. 当前工作目录的 `models/`

显式指定目录但文件缺失时会直接报错，不会静默回退。

## 目录结构

```
models/                     # 统一模型目录
├── ppocr_keys.txt          # 字符字典（入库）
├── paddleocr.rs / layout.rs # onnx2burn 生成代码（入库）
├── paddleocr.onnx / layout.onnx  # 原始模型（gitignore）
└── paddleocr.bpk / layout.bpk    # 转换产物（gitignore）
src/
├── main.rs                 # CLI（clap）
├── server.rs               # HTTP（axum）
├── pipeline.rs             # 图片 → 版面 → 识别 → 文档树
├── layout.rs               # 版面检测（DETR 后处理 + 阅读顺序 + IoU 去重）
├── preprocess.rs           # 去阴影、二值化、去横线、行切分
├── ocr.rs                  # 行识别（CTC 贪心解码）
├── table.rs                # 表格网格还原
├── document.rs             # 文档结构树
├── render/                 # 渲染层（markdown / plain；PDF、DOCX 预留）
└── model.rs                # 生成代码引入 + 权重定位
```

处理流程：

```
图片 → 版面区域 → 按类别分发
                   ├─ text/title → 行切分 → 逐行识别 → 段落
                   ├─ table      → 网格检测 → 单元格识别 → 表格
                   └─ image      → 裁图 → base64 内嵌
        → 文档结构树 → Renderer → Markdown / 纯文本
```

## 发布

```bash
bash scripts/package.sh windows-x64        # 本地打包（权重需已就绪）
git tag v0.0.1 && git push origin v0.0.1   # 触发 CI 构建三平台并发布 Release
```

CI（`.github/workflows/release.yml`）在各自架构的原生环境编译：Windows MSVC、
`ubuntu-latest`、`ubuntu-24.04-arm`，产物为解压即用的压缩包。

Linux 采用 musl target：Rust 自 1.63 起为 musl 提供 self-contained 的 CRT 与 `libc.a`，
`crt-static` 又是默认值，因此无需 `musl-gcc` 之类的工具链也能得到完全静态的二进制。

## 已知限制

- **无线表格**：版面模型多判为普通文本，会退化成连续文本；有线表格的网格还原是准确的
- **合并单元格**：不还原 rowspan/colspan（Markdown 表格本身不支持）
- **公式 / 印章 / 图表**：只保留占位符，未接入对应识别模型
- **标题层级**：只有 `#`（doc_title）与 `##`（paragraph_title）两级
- **多栏阅读顺序**：一层 XY-cut（纵向聚行 + 行内按横坐标排序），三栏以上或复杂嵌套版面可能错序
- **手写**：识别模型官方手写中文指标约 62%，行书连笔会有错字

## 许可

本项目采用 [Apache License 2.0](LICENSE)。

所用模型来自 [PaddleOCR](https://github.com/PaddlePaddle/PaddleOCR)（同样为 Apache-2.0）：
PP-OCRv6_medium_rec、PP-DocLayout_plus-L。
