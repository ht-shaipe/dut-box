//! copyright © htui.tech 2026 - present
//! 主程序
//! created shaipe by 2026-03-05 10:31:05

use dut_box_assets::Assets;
use gpui::*;
use gpui_platform::application;

use dut_box::{Layout, Welcome};
use gpui_component::Root;

/// 基于 GPUI 的桌面应用
fn main() {
    // Windows: 使用 WebView 时必须关闭 Direct Composition，否则无法渲染（与 gpui-component webview 示例一致）
    #[cfg(target_os = "windows")]
    std::env::set_var("GPUI_DISABLE_DIRECT_COMPOSITION", "true");

    // 初始化应用, 并加载资产
    let app = application().with_assets(Assets);

    app.run(|cx: &mut App| {
        gpui_component::init(cx);
        let bounds = Bounds::centered(None, size(px(1400.0), px(900.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..WindowOptions::default()
            },
            |window, cx| {
                // 使用 Layout 包裹 Welcome，顶部显示自定义标题栏
                let layout = cx.new(|cx| {
                    let view = Welcome::view(window, cx);
                    Layout::new("Developer Utility Tool Box", view, window, cx)
                });
                cx.new(|cx| Root::new(layout, window, cx))
            },
        )
        .unwrap();

        // 激活应用
        cx.activate(true);
    });
}
