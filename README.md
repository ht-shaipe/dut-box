# DUT Box - GPUI Demo

基于 GPUI 的简单文本编辑器 Demo。

## 项目结构

```
dut-box/
├── Cargo.toml
├── src/
│   └── main.rs
└── README.md
```

## 运行要求

- Rust 1.75+ (需要支持 async fn in traits)
- macOS 或 Linux (GPUI 目前主要支持这些平台)

## 安装 Rust

```bash
# 使用 rustup 安装
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

## 运行项目

```bash
# 克隆仓库
git clone https://github.com/ht-shaipe/dut-box.git
cd dut-box

# 运行
cargo run

# 发布模式运行 (性能更好)
cargo run --release
```

## 功能特性

- 响应式窗口布局
- 深色主题界面
- 文本内容展示
- 行数和字符统计

## 依赖

- [GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui) - Zed 编辑器的高性能 UI 框架

## 开发计划

- [ ] 添加文本编辑功能
- [ ] 实现光标闪烁
- [ ] 支持文件打开/保存
- [ ] 添加语法高亮
