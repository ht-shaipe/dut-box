//! copyright © htui.tech 2026 - present
//! 主程序
//! created shaipe by 2026-03-05 10:31:05

use gpui::*;

mod pages;
use pages::Welcome;

/// 基于 GPUI 的桌面应用
fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1400.0), px(900.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..WindowOptions::default()
            },
            |_, cx| cx.new(|_| Welcome::new()),
        )
        .unwrap();
        cx.activate(true);
    });
}
