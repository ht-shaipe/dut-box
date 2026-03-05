//! copyright © htui.tech 2026 - present
//!
//! created shaipe by 2026-03-05 13:17:24

use gpui::{actions, Action, SharedString};
use gpui_component::scroll::ScrollbarShow;
use serde::Deserialize;

/// 选择滚动条显示方式
#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = silo, no_json)]
pub struct SelectScrollbarShow(pub ScrollbarShow);

/// 选择语言
#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = silo, no_json)]
pub struct SelectLocale(pub SharedString);

/// 选择字体大小
#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = silo, no_json)]
pub struct SelectFont(pub usize);

/// 选择圆角大小
#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = silo, no_json)]
pub struct SelectRadius(pub usize);

actions!(
    silo,
    [
        About,
        Open,
        Quit,
        ToggleSearch,
        TestAction,
        Tab,
        TabPrev,
        ShowPanelInfo,
        ToggleListActiveHighlight
    ]
);
