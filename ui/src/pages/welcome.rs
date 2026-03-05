use gpui::prelude::*;
use gpui::*;

/// Web 下使用支持中文的字体，否则用等宽字体
#[inline]
fn mono_font() -> &'static str {
    if cfg!(target_family = "wasm") {
        "Noto Sans SC"
    } else {
        "JetBrains Mono"
    }
}

/// 工具卡片数据结构
struct ToolCard {
    name: SharedString,
    category: SharedString,
    description: SharedString,
    icon: SharedString,
    tags: Vec<SharedString>,
    icon_color: (u8, u8, u8),
}

/// 快捷入口数据结构
struct QuickItem {
    label: SharedString,
    icon: SharedString,
    color: (u8, u8, u8),
}

/// 最近使用数据结构
struct RecentItem {
    name: SharedString,
    icon: SharedString,
    time: SharedString,
}

/// 主页面视图
pub struct Welcome {
    search_text: SharedString,
    active_tab: usize,
    tools: Vec<ToolCard>,
    quick_items: Vec<QuickItem>,
    recent_items: Vec<RecentItem>,
}

impl Welcome {
    pub fn new() -> Self {
        Self {
            search_text: SharedString::from(""),
            active_tab: 0,
            tools: vec![
                ToolCard {
                    name: "JSON 格式化".into(),
                    category: "Formatter".into(),
                    description: "美化、压缩、验证 JSON 数据，支持语法高亮和错误提示".into(),
                    icon: "{ }".into(),
                    tags: vec!["JSON".into(), "格式化".into(), "验证".into()],
                    icon_color: (0, 212, 255),
                },
                ToolCard {
                    name: "Base64 编解码".into(),
                    category: "Encoder".into(),
                    description: "文本与 Base64 格式互转，支持图片 Base64 编码".into(),
                    icon: "64".into(),
                    tags: vec!["Base64".into(), "编码".into(), "图片".into()],
                    icon_color: (245, 158, 11),
                },
                ToolCard {
                    name: "URL 编解码".into(),
                    category: "Converter".into(),
                    description: "URL 编码解码工具，支持中文和特殊字符转换".into(),
                    icon: "URL".into(),
                    tags: vec!["URL".into(), "编码".into(), "中文".into()],
                    icon_color: (168, 85, 247),
                },
                ToolCard {
                    name: "密码生成器".into(),
                    category: "Generator".into(),
                    description: "生成高强度随机密码，支持自定义长度和字符集".into(),
                    icon: "🔐".into(),
                    tags: vec!["密码".into(), "安全".into(), "随机".into()],
                    icon_color: (236, 72, 153),
                },
                ToolCard {
                    name: "时间戳转换".into(),
                    category: "Converter".into(),
                    description: "Unix 时间戳与日期时间互转，支持多种格式".into(),
                    icon: "⏱".into(),
                    tags: vec!["时间戳".into(), "日期".into(), "时区".into()],
                    icon_color: (168, 85, 247),
                },
                ToolCard {
                    name: "正则表达式测试".into(),
                    category: "Validator".into(),
                    description: "在线正则表达式测试工具，支持替换和匹配".into(),
                    icon: ".*".into(),
                    tags: vec!["Regex".into(), "匹配".into(), "测试".into()],
                    icon_color: (16, 185, 129),
                },
            ],
            quick_items: vec![
                QuickItem {
                    label: "JSON".into(),
                    icon: "{ }".into(),
                    color: (0, 212, 255),
                },
                QuickItem {
                    label: "Base64".into(),
                    icon: "64".into(),
                    color: (245, 158, 11),
                },
                QuickItem {
                    label: "时间戳".into(),
                    icon: "⏱".into(),
                    color: (236, 72, 153),
                },
                QuickItem {
                    label: "正则".into(),
                    icon: ".*".into(),
                    color: (16, 185, 129),
                },
                QuickItem {
                    label: "颜色".into(),
                    icon: "🎨".into(),
                    color: (245, 158, 11),
                },
                QuickItem {
                    label: "密码".into(),
                    icon: "🔐".into(),
                    color: (239, 68, 68),
                },
            ],
            recent_items: vec![
                RecentItem {
                    name: "JSON 格式化".into(),
                    icon: "{ }".into(),
                    time: "2 分钟前".into(),
                },
                RecentItem {
                    name: "Base64 编解码".into(),
                    icon: "64".into(),
                    time: "15 分钟前".into(),
                },
                RecentItem {
                    name: "时间戳转换".into(),
                    icon: "⏱".into(),
                    time: "1 小时前".into(),
                },
            ],
        }
    }
}

impl Render for Welcome {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // Web 下使用支持中文的字体，避免乱码
        let root_font = if cfg!(target_family = "wasm") {
            "Noto Sans SC"
        } else {
            "Space Grotesk"
        };
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0x0f0f1a))
            .font_family(root_font)
            .child(self.render_header())
            .child(self.render_search())
            .child(self.render_quick_access())
            .child(self.render_tools_section())
            .child(self.render_recent_section())
            .child(self.render_footer())
    }
}

impl Welcome {
    /// 渲染头部
    fn render_header(&self) -> Div {
        div()
            .flex()
            .justify_between()
            .items_center()
            .px_6()
            .py_4()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .w_12()
                            .h_12()
                            .rounded_xl()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_2xl()
                            .bg(rgba(0x00d4ff))
                            .child("🛠️"),
                    )
                    .child(
                        div()
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(FontWeight::BOLD)
                                    .child("DevToolbox"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x888888))
                                    .font_family(mono_font())
                                    .child("v2.0.0 // DEVELOPER UTILITIES"),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(self.render_icon_button("⚙️".into()))
                    .child(self.render_icon_button("🌙".into()))
                    .child(self.render_icon_button("ℹ️".into())),
            )
    }

    fn render_icon_button(&self, icon: SharedString) -> Div {
        div()
            .w_11()
            .h_11()
            .rounded_lg()
            .flex()
            .items_center()
            .justify_center()
            .text_xl()
            .bg(rgba(0xffffff05))
            .border_1()
            .border_color(rgba(0xffffff10))
            .child(icon)
    }

    /// 渲染搜索框
    fn render_search(&self) -> Div {
        div().px_6().pb_6().child(
            div()
                .max_w(px(700.0))
                .mx_auto()
                .relative()
                .child(
                    div()
                        .absolute()
                        .left(px(16.0))
                        .top(px(16.0))
                        .text_xl()
                        .child("🔍"),
                )
                .child(
                    div()
                        .w_full()
                        .px_6()
                        .py_4()
                        .pr_16()
                        .text_lg()
                        .rounded_2xl()
                        .bg(rgba(0xffffff05))
                        .border_1()
                        .border_color(rgba(0xffffff10))
                        .text_color(rgb(0xffffff))
                        .child("搜索工具... (例如: JSON格式化、Base64编码)"),
                )
                .child(
                    div()
                        .absolute()
                        .right(px(16.0))
                        .top(px(14.0))
                        .px_3()
                        .py_1()
                        .rounded_md()
                        .bg(rgba(0xffffff10))
                        .border_1()
                        .border_color(rgba(0xffffff10))
                        .text_sm()
                        .font_family(mono_font())
                        .text_color(rgb(0x888888))
                        .child("⌘K"),
                ),
        )
    }

    /// 渲染快捷入口
    fn render_quick_access(&self) -> Div {
        div()
            .px_6()
            .pb_6()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0x888888))
                    .font_family(mono_font())
                    .mb_4()
                    .child("// 快捷入口"),
            )
            .child(
                div()
                    .grid()
                    .grid_cols(6)
                    .gap_4()
                    .max_w(px(800.0))
                    .mx_auto()
                    .children(
                        self.quick_items
                            .iter()
                            .map(|item| self.render_quick_item(item)),
                    ),
            )
    }

    fn render_quick_item(&self, item: &QuickItem) -> Div {
        let (r, g, b) = item.color;
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_3()
            .p_4()
            .rounded_2xl()
            .bg(rgba(0xffffff05))
            .border_1()
            .border_color(rgba(0xffffff10))
            .child(
                div()
                    .w_12()
                    .h_12()
                    .rounded_xl()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_xl()
                    .bg(rgba(r as u32 * 0x10000 + g as u32 * 0x100 + b as u32))
                    .child(item.icon.clone()),
            )
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(0xaaaaaa))
                    .child(item.label.clone()),
            )
    }

    /// 渲染工具区域
    fn render_tools_section(&self) -> Div {
        let tabs = vec![
            "全部",
            "格式化",
            "编码转换",
            "生成器",
            "验证器",
            "网络",
            "文本",
        ];

        div()
            .flex_1()
            .px_6()
            .pb_6()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0x888888))
                    .font_family(mono_font())
                    .mb_4()
                    .child("// 全部工具"),
            )
            .child(
                div()
                    .flex()
                    .gap_3()
                    .mb_6()
                    .children(tabs.iter().enumerate().map(|(i, &tab)| {
                        let is_active = i == self.active_tab;
                        div()
                            .px_4()
                            .py_2()
                            .rounded_xl()
                            .when(is_active, |this: Div| {
                                this.bg(rgba(0x00d4ff33)).border_color(rgb(0x00d4ff))
                            })
                            .when(!is_active, |this| {
                                this.bg(rgba(0xffffff05)).border_color(rgba(0xffffff10))
                            })
                            .border_1()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .when(is_active, |this: Div| this.text_color(rgb(0x00d4ff)))
                            .when(!is_active, |this: Div| this.text_color(rgb(0xaaaaaa)))
                            .child(tab)
                    })),
            )
            .child(
                div()
                    .grid()
                    .grid_cols(3)
                    .gap_4()
                    .children(self.tools.iter().map(|tool| self.render_tool_card(tool))),
            )
    }

    fn render_tool_card(&self, tool: &ToolCard) -> Div {
        let (r, g, b) = tool.icon_color;
        div()
            .p_5()
            .rounded_2xl()
            .bg(rgba(0xffffff05))
            .border_1()
            .border_color(rgba(0xffffff10))
            .child(
                div()
                    .flex()
                    .items_start()
                    .gap_3()
                    .mb_3()
                    .child(
                        div()
                            .w_12()
                            .h_12()
                            .rounded_2xl()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_2xl()
                            .bg(rgba(r as u32 * 0x10000 + g as u32 * 0x100 + b as u32))
                            .child(tool.icon.clone()),
                    )
                    .child(
                        div()
                            .flex_1()
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0xffffff))
                                    .child(tool.name.clone()),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x888888))
                                    .font_family(mono_font())
                                    .child(tool.category.clone()),
                            ),
                    ),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0xaaaaaa))
                    .mb_3()
                    .child(tool.description.clone()),
            )
            .child(div().flex().gap_2().children(tool.tags.iter().map(|tag| {
                div()
                    .px_3()
                    .py_1()
                    .rounded_full()
                    .bg(rgba(0xffffff05))
                    .border_1()
                    .border_color(rgba(0xffffff10))
                    .text_xs()
                    .text_color(rgb(0x888888))
                    .font_family(mono_font())
                    .child(tag.clone())
            })))
    }

    /// 渲染最近使用
    fn render_recent_section(&self) -> Div {
        div()
            .px_6()
            .pb_6()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0x888888))
                    .font_family(mono_font())
                    .mb_4()
                    .child("// 最近使用"),
            )
            .child(
                div().flex().flex_col().gap_3().children(
                    self.recent_items
                        .iter()
                        .map(|item| self.render_recent_item(item)),
                ),
            )
    }

    fn render_recent_item(&self, item: &RecentItem) -> Div {
        div()
            .flex()
            .items_center()
            .gap_4()
            .px_5()
            .py_4()
            .rounded_xl()
            .bg(rgba(0xffffff05))
            .border_1()
            .border_color(rgba(0xffffff10))
            .child(
                div()
                    .w_10()
                    .h_10()
                    .rounded_lg()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_xl()
                    .bg(rgba(0xffffff10))
                    .child(item.icon.clone()),
            )
            .child(
                div()
                    .flex_1()
                    .child(
                        div()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(0xffffff))
                            .child(item.name.clone()),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x888888))
                            .font_family(mono_font())
                            .child(item.time.clone()),
                    ),
            )
            .child(div().text_xl().text_color(rgb(0x888888)).child("→"))
    }

    /// 渲染页脚
    fn render_footer(&self) -> Div {
        div()
            .px_6()
            .py_6()
            .border_t_1()
            .border_color(rgba(0xffffff10))
            .child(
                div()
                    .text_center()
                    .text_sm()
                    .text_color(rgb(0x888888))
                    .child("Made with ")
                    .child(div().text_color(rgb(0xec4899)).child("♥"))
                    .child(" for Developers"),
            )
    }
}
