//! copyright © htui.tech 2026 - present
//!
//! created shaipe by 2026-03-05 14:12:17

use gpui::{
    div, px, size, AnyView, App, Bounds, Context, Entity, FocusHandle, IntoElement, ParentElement,
    Render, SharedString, Task, Window, WindowBounds, WindowHandle, WindowKind, WindowOptions,
};
use gpui::{AppContext as _, InteractiveElement, Styled};
use gpui_component::Root;

use crate::app::AppTitleBar;

pub struct Layout {
    // 焦点处理
    pub(crate) focus_handle: FocusHandle,
    // 标题栏
    pub(crate) title_bar: Entity<AppTitleBar>,
    // 视图
    pub(crate) view: AnyView,
}

impl Layout {
    pub fn new(
        title: impl Into<SharedString>,
        view: impl Into<AnyView>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let title_bar = cx.new(|cx| AppTitleBar::new(title, window, cx));
        Self {
            focus_handle: cx.focus_handle(),
            title_bar,
            view: view.into(),
        }
    }

    /// 创建新的本地窗口（参考 crawler 工程中的 `DockWorkspace::new_local`）
    pub fn new_local(cx: &mut App) -> Task<anyhow::Result<WindowHandle<Root>>> {
        // 获取窗口大小
        let mut window_size = size(px(1600.0), px(1200.0));
        // 获取主显示器大小，限制窗口尺寸不超过 85%
        if let Some(display) = cx.primary_display() {
            let display_size = display.bounds().size;
            // 窗口宽度不能超过主显示器宽度的 85%
            window_size.width = window_size.width.min(display_size.width * 0.85);
            // 窗口高度不能超过主显示器高度的 85%
            window_size.height = window_size.height.min(display_size.height * 0.85);
        }
        // 创建窗口边界
        let window_bounds = Bounds::centered(None, window_size, cx);

        cx.spawn(async move |cx| {
            let title = "Developer Utility Tool Box";

            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(window_bounds)),
                #[cfg(not(target_os = "linux"))]
                titlebar: Some(gpui_component::TitleBar::title_bar_options()),
                window_min_size: Some(gpui::Size {
                    width: px(640.0),
                    height: px(480.0),
                }),
                #[cfg(target_os = "linux")]
                window_background: gpui::WindowBackgroundAppearance::Opaque,
                #[cfg(target_os = "linux")]
                window_decorations: Some(gpui::WindowDecorations::Client),
                kind: WindowKind::Normal,
                ..Default::default()
            };

            let window = cx
                .open_window(options, |window, cx| {
                    // 这里可以根据需要替换为实际的根视图（目前先使用 Welcome 布局）
                    let layout = cx.new(|cx| {
                        let view = crate::pages::Welcome::view(window, cx);
                        Layout::new(title, view, window, cx)
                    });

                    cx.new(|cx| Root::new(layout, window, cx))
                })
                .expect("failed to open window");

            window
                .update(cx, |_, window, _| {
                    window.activate_window();
                    window.set_window_title(title);
                })
                .expect("failed to update window");

            Ok(window)
        })
    }
}

impl Render for Layout {
    /// 渲染 Layout：上方标题栏 + 下方内容视图
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("dut-box-layout")
            .size_full()
            .flex()
            .flex_col()
            .child(self.title_bar.clone())
            .child(self.view.clone())
    }
}
