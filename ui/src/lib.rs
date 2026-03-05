//! copyright © htui.tech 2026 - present
//! 页面模块
//! created shaipe by 2026-03-05 10:31:05

rust_i18n::i18n!("locales", fallback = "en");

//
pub mod app;

pub mod layout;
use gpui_component::{text::markdown, Root, WindowExt};
pub use layout::Layout;
//
pub mod pages;
use gpui::{App, KeyBinding};
pub use pages::{TaskManager, Welcome};

use app::embedded_themes;

use crate::app::{About, ToggleSearch};

use crate::app::{Open, Quit};

/// 初始化 gpui_component（Theme 等），桌面端与 web 在 open_window 前必须调用
pub fn init_gpui_component(cx: &mut gpui::App) {
    gpui_component::init(cx);
}

/// 初始化应用
pub fn init(cx: &mut App) {
    // Try to initialize tracing subscriber, but ignore if already initialized
    #[cfg(not(target_family = "wasm"))]
    {
        use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};
        // 初始化 tracing
        let _ = tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .with(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive("gpui_component=trace".parse().unwrap()),
            )
            .try_init();
    }

    // For WASM, use a subscriber without time support
    #[cfg(target_family = "wasm")]
    {
        use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};
        // 初始化 tracing
        let _ = tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().without_time())
            .with(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive("gpui_component=trace".parse().unwrap()),
            )
            .try_init();
    }

    // 初始化 gpui_component
    gpui_component::init(cx);
    // 初始化应用状态
    app::AppState::init(cx);
    // 初始化主题
    app::themes::init(cx);
    // 初始化页面
    pages::init(cx);

    #[cfg(not(target_family = "wasm"))]
    {
        let http_client = reqwest_client::ReqwestClient::user_agent("htui/dut-box").unwrap();
        cx.set_http_client(std::sync::Arc::new(http_client));
    }

    #[cfg(target_family = "wasm")]
    {
        // Safety: the web examples run single-threaded; the client is
        // created and used exclusively on the main thread.
        let http_client = unsafe {
            gpui_web::FetchHttpClient::with_user_agent("htui/dut-box")
                .expect("failed to create FetchHttpClient")
        };
        cx.set_http_client(std::sync::Arc::new(http_client));
    }

    // 绑定快捷键
    cx.bind_keys([
        KeyBinding::new("/", ToggleSearch, None),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-o", Open, None),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-o", Open, None),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-q", Quit, None),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("alt-f4", Quit, None),
    ]);

    // 退出应用
    cx.on_action(|_: &Quit, cx: &mut App| {
        cx.quit();
    });

    // 关于应用
    cx.on_action(|_: &About, cx: &mut App| {
        if let Some(window) = cx.active_window().and_then(|w| w.downcast::<Root>()) {
            cx.defer(move |cx| {
                window
                    .update(cx, |_, window, cx| {
                        window.defer(cx, |window, cx| {
                            window.open_alert_dialog(cx, |alert, _, _| {
                                alert.title("About").description(markdown(
                                    "GPUI Component Storybook\n\n\
                                    Version 0.1.0\n\n\
                                    https://longbridge.github.io/gpui-component",
                                ))
                            });
                        });
                    })
                    .unwrap();
            });
        }
    });

    // 注册面板
    // register_panel(cx, PANEL_NAME, |_, _, info, window, cx| {
    //     let story_state = match info {
    //         PanelInfo::Panel(value) => StoryState::from_value(value.clone()),
    //         _ => {
    //             unreachable!("Invalid PanelInfo: {:?}", info)
    //         }
    //     };

    //     // 创建面板视图
    //     let view = cx.new(|cx| {
    //         let (title, description, closable, zoomable, story, on_active) =
    //             story_state.to_story(window, cx);
    //         let mut container = StoryContainer::new(window, cx)
    //             .story(story, story_state.story_klass)
    //             .on_active(on_active);

    //         cx.on_focus_in(
    //             &container.focus_handle,
    //             window,
    //             |this: &mut StoryContainer, _, _| {
    //                 println!("StoryContainer focus in: {}", this.name);
    //             },
    //         )
    //         .detach();

    //         container.name = title.into();
    //         container.description = description.into();
    //         container.closable = closable;
    //         container.zoomable = zoomable;
    //         container
    //     });
    //     Box::new(view)
    // });

    // 激活应用
    cx.activate(true);
}
