//! copyright © htui.tech 2026 - present
//!
//! created shaipe by 2026-03-05 13:23:41

use gpui::{App, AppContext, Entity, Global, SharedString};

/// 应用状态
pub struct AppState {
    pub invisible_panels: Entity<Vec<SharedString>>,
}

/// 应用状态实现
impl AppState {
    /// 初始化应用状态
    pub fn init(cx: &mut App) {
        let state = Self {
            invisible_panels: cx.new(|_| Vec::new()),
        };
        cx.set_global::<AppState>(state);
    }

    /// 获取应用状态
    pub fn global(cx: &App) -> &Self {
        cx.global::<Self>()
    }

    /// 获取应用状态可变引用
    pub fn global_mut(cx: &mut App) -> &mut Self {
        cx.global_mut::<Self>()
    }
}

/// 应用状态实现 Global trait
impl Global for AppState {}
