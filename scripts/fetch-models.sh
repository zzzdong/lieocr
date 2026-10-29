#!/usr/bin/env bash
# 下载 ONNX 模型并转换为 Burn 权重（.bpk）。
#
# `models/` 是统一的模型目录：ONNX 源文件、onnx2burn 生成的 .rs/.bpk、字符字典都放这里。
# 权重与 ONNX 都不进入 Git，因此首次构建需要执行本脚本重建。
# 重复执行是安全的：已存在且校验通过的 ONNX 会跳过下载。
#
# 依赖：curl、sha256sum、onnx2burn（cargo install burn-onnx --version 0.22.0）

set -euo pipefail
cd "$(dirname "$0")/.."

BASE="https://www.modelscope.cn/models/PaddlePaddle"
REC_SHA="9c09abf0957f7968c7586464b7397b84ad2387a0497a351af40e9acc71b673ba"
LAYOUT_SHA="77afb2caa74dd13240d087d2eced91d7fcd2caebd16006a0a66162fc8707ff0e"

mkdir -p models

fetch() { # $1=url $2=目标文件 $3=sha256
    if [ -f "$2" ] && echo "$3  $2" | sha256sum -c --status 2>/dev/null; then
        echo "✓ $2（已存在，校验通过）"
        return
    fi
    rm -f "$2"
    echo "↓ $2"
    curl -L -C - --retry 5 --retry-delay 3 -o "$2" "$1"
    echo "$3  $2" | sha256sum -c -
}

fetch "$BASE/PP-OCRv6_medium_rec_onnx/resolve/master/inference.onnx" \
    models/paddleocr.onnx "$REC_SHA"
fetch "$BASE/PP-DocLayout_plus-L_onnx/resolve/master/inference.onnx" \
    models/layout.onnx "$LAYOUT_SHA"

if ! command -v onnx2burn >/dev/null 2>&1; then
    echo "未找到 onnx2burn，正在安装 burn-onnx 0.22 …"
    cargo install burn-onnx --version 0.22.0
fi

# 生成 models/{paddleocr,layout}.{rs,bpk}
onnx2burn models/paddleocr.onnx models --no-development

# 版面模型必须关掉图简化，否则 burn-onnx 会在 attention 融合处 panic
onnx2burn models/layout.onnx models --no-development --no-simplify

echo
echo "完成："
ls -lh models/*.bpk models/*.rs
