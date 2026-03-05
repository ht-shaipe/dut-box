#!/bin/bash
# WASM 构建说明：
# 当前 gpui 0.2 依赖链会拉入 rustix/calloop 等仅支持原生平台的库，
# 在 wasm32-unknown-unknown 下会报错（如 cannot find `zero_msghdr` in module `c`）。
# 若构建失败，可先仅跑桌面端：在项目根目录执行 cargo run -p dut-box
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Building Web...${NC}"

# Get the script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$SCRIPT_DIR/.."

# Parse arguments
RELEASE_FLAG=""
if [[ "$1" == "--release" ]]; then
    RELEASE_FLAG="--release"
    echo -e "${YELLOW}Building in release mode${NC}"
fi

# workspace 根目录 = web 的上一级 (dut-box)
WORKSPACE_ROOT="$PROJECT_ROOT/.."

# Step 1: Build WASM（需 nightly：gpui_web 依赖的 wasm_thread 使用 stdarch_wasm_atomic_wait）
echo -e "${GREEN}Step 1: Building WASM...${NC}"
cd "$WORKSPACE_ROOT"
cargo +nightly build -p dut-box-web --target wasm32-unknown-unknown $RELEASE_FLAG

# Determine the build directory
if [[ "$RELEASE_FLAG" == "--release" ]]; then
    BUILD_MODE="release"
else
    BUILD_MODE="debug"
fi

# WASM file is in workspace target directory
WASM_PATH="$WORKSPACE_ROOT/target/wasm32-unknown-unknown/$BUILD_MODE/dut_box_web.wasm"

# Check if WASM file exists
if [[ ! -f "$WASM_PATH" ]]; then
    echo -e "${RED}Error: WASM file not found at: $WASM_PATH${NC}"
    exit 1
fi

# Step 2: Generate JavaScript bindings
echo -e "${GREEN}Step 2: Generating JavaScript bindings...${NC}"
wasm-bindgen "$WASM_PATH" \
    --out-dir "$PROJECT_ROOT/www/src/wasm" \
    --target web \
    --no-typescript

echo -e "${GREEN}✓ Build completed successfully!${NC}"
echo -e "${YELLOW}Next steps:${NC}"
echo -e "  cd www"
echo -e "  bun install"
echo -e "  bun run dev"
