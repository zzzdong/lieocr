#!/usr/bin/env bash
# 生成发布包：dist/lieocr-v<version>-<platform>.{zip,tar.gz}
#
# 用法：
#   scripts/package.sh [platform]
#
#   platform: windows-x64 | linux-x64-musl | linux-aarch64-musl
#             （省略时按当前系统推断）
#   BIN_DIR:  可执行文件目录，默认 target/release；
#             CI 交叉编译时传 target/<triple>/release
#
# 包内布局（解压即用，无需安装）：
#   lieocr-v0.1.0-<platform>/
#   ├── lieocr(.exe)
#   ├── models/paddleocr.bpk
#   ├── models/layout.bpk
#   └── README.md / LICENSE（若存在）
#
# exe 会优先从「同级 models/」加载权重。
# 权重不进 Git，打包前请先执行 scripts/fetch-models.sh。

set -euo pipefail
cd "$(dirname "$0")/.."

PLATFORM="${1:-}"
if [ -z "$PLATFORM" ]; then
    case "$(uname -s)" in
        MINGW* | MSYS* | CYGWIN*) PLATFORM="windows-x64" ;;
        *) PLATFORM="linux-x64-musl" ;;
    esac
fi

case "$PLATFORM" in
    windows-*) BIN_NAME="lieocr.exe" ;;
    *) BIN_NAME="lieocr" ;;
esac

VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)
NAME="lieocr-v${VERSION}-${PLATFORM}"
DIST="dist/${NAME}"
BIN_DIR="${BIN_DIR:-target/release}"
BIN="${BIN_DIR}/${BIN_NAME}"

for f in models/paddleocr.bpk models/layout.bpk; do
    if [ ! -f "$f" ]; then
        echo "缺少 $f，请先运行 scripts/fetch-models.sh" >&2
        exit 1
    fi
done

if [ ! -f "$BIN" ]; then
    echo "找不到可执行文件 $BIN" >&2
    echo "本地打包请先 cargo build --release；交叉编译请设置 BIN_DIR=target/<triple>/release" >&2
    exit 1
fi

rm -rf "$DIST"
mkdir -p "$DIST/models"
cp "$BIN" "$DIST/$BIN_NAME"
chmod +x "$DIST/$BIN_NAME" 2>/dev/null || true
cp models/paddleocr.bpk models/layout.bpk "$DIST/models/"
for doc in README.md LICENSE; do
    [ -f "$doc" ] && cp "$doc" "$DIST/"
done

cd dist
if [ "$PLATFORM" = "windows-x64" ] && command -v powershell >/dev/null 2>&1; then
    powershell -NoProfile -Command "Compress-Archive -Path '${NAME}' -DestinationPath '${NAME}.zip' -Force" >/dev/null
    echo "→ dist/${NAME}.zip（$(du -h "${NAME}.zip" | cut -f1)）"
elif command -v zip >/dev/null 2>&1; then
    zip -qr "${NAME}.zip" "${NAME}"
    echo "→ dist/${NAME}.zip（$(du -h "${NAME}.zip" | cut -f1)）"
else
    tar -czf "${NAME}.tar.gz" "${NAME}"
    echo "→ dist/${NAME}.tar.gz（$(du -h "${NAME}.tar.gz" | cut -f1)）"
fi
