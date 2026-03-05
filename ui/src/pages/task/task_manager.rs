use gpui::prelude::FluentBuilder;
use gpui::*;

/// 任务状态
#[derive(Clone, Copy, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

impl TaskStatus {
    pub fn label(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "待执行",
            TaskStatus::Running => "运行中",
            TaskStatus::Completed => "已完成",
            TaskStatus::Failed => "失败",
        }
    }

    pub fn color(&self) -> Rgba {
        match self {
            TaskStatus::Pending => rgba(0x999999ff),
            TaskStatus::Running => rgba(0x0066ffff),
            TaskStatus::Completed => rgba(0x10b981ff),
            TaskStatus::Failed => rgba(0xef4444ff),
        }
    }

    /// 状态背景色（约 10% 透明度）
    pub fn bg_alpha(&self) -> Rgba {
        match self {
            TaskStatus::Pending => rgba(0x99999919),
            TaskStatus::Running => rgba(0x0066ff19),
            TaskStatus::Completed => rgba(0x10b98119),
            TaskStatus::Failed => rgba(0xef444419),
        }
    }
}

fn status_bg_alpha(status: TaskStatus) -> Rgba {
    status.bg_alpha()
}

/// 任务数据结构
pub struct Task {
    pub id: SharedString,
    pub name: SharedString,
    pub url: SharedString,
    pub schedule: SharedString,
    pub creator: SharedString,
    pub status: TaskStatus,
    pub collected: SharedString,
    pub success_rate: SharedString,
    pub env_tag: SharedString,
}

/// 统计数据
pub struct StatData {
    pub label: SharedString,
    pub value: SharedString,
    pub change: SharedString,
    pub icon: SharedString,
    pub icon_bg: Rgba,
}

/// 任务管理器视图
pub struct TaskManager {
    pub active_filter: usize,
    pub tasks: Vec<Task>,
    pub stats: Vec<StatData>,
    pub selected_task: Option<SharedString>,
    pub show_detail_panel: bool,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            active_filter: 0,
            tasks: vec![
                Task {
                    id: "task1".into(),
                    name: "电商商品价格采集".into(),
                    url: "https://api.example.com/tasks".into(),
                    schedule: "每 30 分钟".into(),
                    creator: "Admin".into(),
                    status: TaskStatus::Running,
                    collected: "2.4k".into(),
                    success_rate: "99%".into(),
                    env_tag: "PROD".into(),
                },
                Task {
                    id: "task2".into(),
                    name: "新闻资讯抓取".into(),
                    url: "https://news.example.com/feed".into(),
                    schedule: "每小时".into(),
                    creator: "Sarah".into(),
                    status: TaskStatus::Pending,
                    collected: "0".into(),
                    success_rate: "-".into(),
                    env_tag: "TEST".into(),
                },
                Task {
                    id: "task3".into(),
                    name: "用户评论数据同步".into(),
                    url: "https://comments.api.internal".into(),
                    schedule: "每天 02:00".into(),
                    creator: "Mike".into(),
                    status: TaskStatus::Completed,
                    collected: "15.6k".into(),
                    success_rate: "100%".into(),
                    env_tag: "PROD".into(),
                },
                Task {
                    id: "task4".into(),
                    name: "社交媒体趋势分析".into(),
                    url: "https://social.api.example".into(),
                    schedule: "每 15 分钟".into(),
                    creator: "John".into(),
                    status: TaskStatus::Failed,
                    collected: "128".into(),
                    success_rate: "45%".into(),
                    env_tag: "DEV".into(),
                },
                Task {
                    id: "task5".into(),
                    name: "库存数据监控".into(),
                    url: "https://inventory.internal/api".into(),
                    schedule: "每 5 分钟".into(),
                    creator: "Admin".into(),
                    status: TaskStatus::Running,
                    collected: "8.9k".into(),
                    success_rate: "99.8%".into(),
                    env_tag: "PROD".into(),
                },
            ],
            stats: vec![
                StatData {
                    label: "总任务数".into(),
                    value: "48".into(),
                    change: "+5 本周新增".into(),
                    icon: "📋".into(),
                    icon_bg: rgba(0xe6f0ffff),
                },
                StatData {
                    label: "运行中".into(),
                    value: "6".into(),
                    change: "平均耗时 2.4s".into(),
                    icon: "▶️".into(),
                    icon_bg: rgba(0xfef3c7ff),
                },
                StatData {
                    label: "今日成功".into(),
                    value: "1,284".into(),
                    change: "98.2% 成功率".into(),
                    icon: "✓".into(),
                    icon_bg: rgba(0xd1fae5ff),
                },
                StatData {
                    label: "失败任务".into(),
                    value: "3".into(),
                    change: "需关注".into(),
                    icon: "✕".into(),
                    icon_bg: rgba(0xfee2e2ff),
                },
            ],
            selected_task: None,
            show_detail_panel: false,
        }
    }

    fn filtered_tasks(&self) -> Vec<&Task> {
        match self.active_filter {
            0 => self.tasks.iter().collect(),
            1 => self
                .tasks
                .iter()
                .filter(|t| matches!(t.status, TaskStatus::Running))
                .collect(),
            2 => self
                .tasks
                .iter()
                .filter(|t| matches!(t.status, TaskStatus::Pending))
                .collect(),
            3 => self
                .tasks
                .iter()
                .filter(|t| matches!(t.status, TaskStatus::Completed))
                .collect(),
            4 => self
                .tasks
                .iter()
                .filter(|t| matches!(t.status, TaskStatus::Failed))
                .collect(),
            _ => self.tasks.iter().collect(),
        }
    }
}

impl Render for TaskManager {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .bg(rgb(0xfafafa))
            .font_family("Plus Jakarta Sans")
            .child(self.render_sidebar())
            .child(self.render_main_content(cx))
            .when(self.show_detail_panel, |this: Div| {
                this.child(self.render_detail_panel(cx))
            })
    }
}

impl TaskManager {
    /// 渲染侧边栏
    fn render_sidebar(&self) -> Div {
        div()
            .w(px(240.0))
            .h_full()
            .flex()
            .flex_col()
            .bg(rgb(0xffffff))
            .border_r_1()
            .border_color(rgb(0xe8e8e8))
            .child(
                // Logo
                div()
                    .p(px(24.0))
                    .border_b_1()
                    .border_color(rgb(0xe8e8e8))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .w(px(36.0))
                                    .h(px(36.0))
                                    .rounded_md()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .bg(rgb(0x0066ff))
                                    .text_color(rgb(0xffffff))
                                    .font_weight(FontWeight::BOLD)
                                    .child("TF"),
                            )
                            .child(
                                div()
                                    .child(div().font_weight(FontWeight::BOLD).child("TaskFlow"))
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x999999))
                                            .font_family("IBM Plex Mono")
                                            .child("v2.4.0"),
                                    ),
                            ),
                    ),
            )
            .child(
                // Navigation
                div()
                    .flex_1()
                    .p(px(16.0))
                    .child(self.render_nav_section(
                        "主菜单",
                        vec![
                            ("📋", "任务管理", true, Some("12")),
                            ("⚙️", "执行器配置", false, None),
                            ("📊", "数据报表", false, None),
                            ("🔌", "API 设置", false, None),
                        ],
                    ))
                    .child(self.render_nav_section(
                        "监控",
                        vec![
                            ("📈", "执行日志", false, None),
                            ("🔔", "告警通知", false, None),
                        ],
                    )),
            )
            .child(
                // User info
                div()
                    .p(px(16.0))
                    .border_t_1()
                    .border_color(rgb(0xe8e8e8))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .w(px(36.0))
                                    .h(px(36.0))
                                    .rounded_full()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .bg(rgb(0x667eea))
                                    .text_color(rgb(0xffffff))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("JD"),
                            )
                            .child(
                                div()
                                    .child(
                                        div()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_sm()
                                            .child("John Doe"),
                                    )
                                    .child(
                                        div().text_xs().text_color(rgb(0x999999)).child("管理员"),
                                    ),
                            ),
                    ),
            )
    }

    fn render_nav_section(&self, title: &str, items: Vec<(&str, &str, bool, Option<&str>)>) -> Div {
        let title_div = div()
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(rgb(0x999999))
            .mb(px(8.0))
            .child(title.to_string());

        let items_div = div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .children(items.into_iter().map(|(icon, label, active, badge)| {
                let item = div()
                    .flex()
                    .items_center()
                    .gap(px(12.0))
                    .px(px(12.0))
                    .py(px(10.0))
                    .rounded_sm()
                    .when(active, |this: Div| {
                        this.bg(rgb(0xe6f0ff)).text_color(rgb(0x0066ff))
                    })
                    .when(!active, |this: Div| {
                        this.text_color(rgb(0x666666))
                            .hover(|style| style.bg(rgb(0xf5f5f5)).text_color(rgb(0x111111)))
                    })
                    .child(
                        div()
                            .w(px(20.0))
                            .h(px(20.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(icon.to_string()),
                    )
                    .child(
                        div()
                            .flex_1()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .child(label.to_string()),
                    );
                if let Some(b) = badge {
                    item.child(
                        div()
                            .px(px(8.0))
                            .py(px(2.0))
                            .rounded_full()
                            .bg(rgb(0x0066ff))
                            .text_color(rgb(0xffffff))
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .font_family("IBM Plex Mono")
                            .child(b.to_string()),
                    )
                } else {
                    item
                }
            }));

        div().mb(px(24.0)).child(title_div).child(items_div)
    }

    /// 渲染主内容区
    fn render_main_content(&self, cx: &mut Context<Self>) -> Div {
        div()
            .flex_1()
            .flex()
            .flex_col()
            .child(self.render_header(cx))
            .child(self.render_content())
    }

    fn render_header(&self, _cx: &mut Context<Self>) -> Div {
        div()
            .h(px(72.0))
            .bg(rgb(0xffffff))
            .border_b_1()
            .border_color(rgb(0xe8e8e8))
            .flex()
            .items_center()
            .justify_between()
            .px(px(32.0))
            .child(
                div()
                    .text_xl()
                    .font_weight(FontWeight::BOLD)
                    .child("数据采集任务"),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(12.0))
                    .child(
                        div()
                            .px(px(16.0))
                            .py(px(10.0))
                            .rounded_sm()
                            .border_1()
                            .border_color(rgb(0x0066ff))
                            .text_color(rgb(0x0066ff))
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .cursor_pointer()
                            .child("📥 导入配置"),
                    )
                    .child(
                        div()
                            .px(px(16.0))
                            .py(px(10.0))
                            .rounded_sm()
                            .bg(rgb(0x0066ff))
                            .text_color(rgb(0xffffff))
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .cursor_pointer()
                            .child("+ 新建任务"),
                    )
                    .child(self.render_icon_button("🔔".into()))
                    .child(self.render_icon_button("❓".into())),
            )
    }

    fn render_icon_button(&self, icon: SharedString) -> Div {
        div()
            .w(px(40.0))
            .h(px(40.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_sm()
            .border_1()
            .border_color(rgb(0xe8e8e8))
            .text_color(rgb(0x666666))
            .text_xl()
            .hover(|style| style.bg(rgb(0xf5f5f5)).text_color(rgb(0x111111)))
            .child(icon)
    }

    fn render_content(&self) -> Div {
        div()
            .flex_1()
            .p(px(32.0))
            .child(self.render_stats_grid())
            .child(self.render_task_section())
    }

    fn render_stats_grid(&self) -> Div {
        div()
            .grid()
            .grid_cols(4)
            .gap(px(20.0))
            .mb(px(32.0))
            .children(self.stats.iter().map(|stat| {
                div()
                    .bg(rgb(0xffffff))
                    .border_1()
                    .border_color(rgb(0xe8e8e8))
                    .rounded_lg()
                    .p(px(24.0))
                    .hover(|style| style.shadow_md())
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .mb(px(12.0))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0x666666))
                                    .font_weight(FontWeight::MEDIUM)
                                    .child(stat.label.clone()),
                            )
                            .child(
                                div()
                                    .w(px(36.0))
                                    .h(px(36.0))
                                    .rounded_sm()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .bg(stat.icon_bg)
                                    .text_lg()
                                    .child(stat.icon.clone()),
                            ),
                    )
                    .child(
                        div()
                            .text_3xl()
                            .font_weight(FontWeight::BOLD)
                            .mb(px(4.0))
                            .child(stat.value.clone()),
                    )
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x10b981))
                            .flex()
                            .items_center()
                            .gap(px(4.0))
                            .child(stat.change.clone()),
                    )
            }))
    }

    fn render_task_section(&self) -> Div {
        let filters = vec![
            ("全部", 48usize),
            ("运行中", 6usize),
            ("待执行", 12usize),
            ("已完成", 27usize),
            ("失败", 3usize),
        ];

        div()
            .bg(rgb(0xffffff))
            .border_1()
            .border_color(rgb(0xe8e8e8))
            .rounded_lg()
            .child(
                // Section header
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .p(px(24.0))
                    .border_b_1()
                    .border_color(rgb(0xe8e8e8))
                    .child(
                        div()
                            .text_base()
                            .font_weight(FontWeight::BOLD)
                            .child("任务列表"),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(12.0))
                            .child(self.render_search_box())
                            .child(
                                div()
                                    .px(px(16.0))
                                    .py(px(10.0))
                                    .rounded_sm()
                                    .border_1()
                                    .border_color(rgb(0x0066ff))
                                    .text_color(rgb(0x0066ff))
                                    .text_sm()
                                    .cursor_pointer()
                                    .child("筛选"),
                            ),
                    ),
            )
            .child(
                // Filter tabs
                div()
                    .flex()
                    .gap(px(4.0))
                    .px(px(24.0))
                    .border_b_1()
                    .border_color(rgb(0xe8e8e8))
                    .children(filters.into_iter().enumerate().map(|(i, (label, count))| {
                        let is_active = i == self.active_filter;
                        div()
                            .px(px(16.0))
                            .py(px(12.0))
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .when(is_active, |this: Div| {
                                this.text_color(rgb(0x0066ff))
                                    .border_b_2()
                                    .border_color(rgb(0x0066ff))
                            })
                            .when(!is_active, |this: Div| {
                                this.text_color(rgb(0x666666))
                                    .hover(|style| style.text_color(rgb(0x111111)))
                            })
                            .cursor_pointer()
                            .child(label)
                            .child(
                                div()
                                    .ml(px(6.0))
                                    .px(px(8.0))
                                    .py(px(2.0))
                                    .rounded_full()
                                    .bg(rgb(0xf5f5f5))
                                    .text_xs()
                                    .font_family("IBM Plex Mono")
                                    .child(count.to_string()),
                            )
                    })),
            )
            .child(
                // Task list
                div().children(
                    self.filtered_tasks()
                        .iter()
                        .map(|task| self.render_task_item(task)),
                ),
            )
    }

    fn render_search_box(&self) -> Div {
        div()
            .relative()
            .child(
                div()
                    .absolute()
                    .left(px(14.0))
                    .top(px(10.0))
                    .text_color(rgb(0x999999))
                    .child("🔍"),
            )
            .child(
                div()
                    .w(px(280.0))
                    .px(px(14.0))
                    .py(px(10.0))
                    .pl(px(40.0))
                    .border_1()
                    .border_color(rgb(0xe8e8e8))
                    .rounded_sm()
                    .text_sm()
                    .bg(rgb(0xf5f5f5))
                    .text_color(rgb(0x999999))
                    .child("搜索任务名称..."),
            )
    }

    fn render_task_item(&self, task: &Task) -> Div {
        let status_color = task.status.color();
        let is_prod = task.env_tag.as_ref() == "PROD";

        div()
            .flex()
            .items_center()
            .px(px(24.0))
            .py(px(16.0))
            .border_b_1()
            .border_color(rgb(0xe8e8e8))
            .hover(|style| style.bg(rgb(0xf5f5f5)))
            .child(
                // Checkbox
                div()
                    .w(px(18.0))
                    .h(px(18.0))
                    .mr(px(16.0))
                    .border_2()
                    .border_color(rgb(0xd0d0d0))
                    .rounded(px(4.0))
                    .cursor_pointer()
                    .hover(|style| style.border_color(rgb(0x0066ff))),
            )
            .child(
                // Task info
                div()
                    .flex_1()
                    .min_w(px(0.0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(10.0))
                            .mb(px(4.0))
                            .child(
                                div()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(task.name.clone()),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .px(px(8.0))
                                    .py(px(2.0))
                                    .rounded(px(4.0))
                                    .when(is_prod, |this: Div| {
                                        this.bg(rgb(0xe6f0ff)).text_color(rgb(0x0066ff))
                                    })
                                    .when(!is_prod, |this: Div| {
                                        this.bg(rgb(0xf5f5f5)).text_color(rgb(0x999999))
                                    })
                                    .font_weight(FontWeight::MEDIUM)
                                    .child(task.env_tag.clone()),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(16.0))
                            .text_xs()
                            .text_color(rgb(0x999999))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .child("🌐")
                                    .child(task.url.clone()),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .child("⏱")
                                    .child(task.schedule.clone()),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .child("👤")
                                    .child(format!("创建者: {}", task.creator)),
                            ),
                    ),
            )
            .child(
                // Status
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .px(px(12.0))
                    .py(px(6.0))
                    .rounded_full()
                    .mr(px(16.0))
                    .bg(status_bg_alpha(task.status))
                    .text_color(status_color)
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(div().w(px(6.0)).h(px(6.0)).rounded_full().bg(status_color))
                    .child(task.status.label()),
            )
            .child(
                // Stats
                div()
                    .flex()
                    .items_center()
                    .gap(px(24.0))
                    .mr(px(16.0))
                    .child(
                        div()
                            .text_center()
                            .child(
                                div()
                                    .font_weight(FontWeight::BOLD)
                                    .text_sm()
                                    .font_family("IBM Plex Mono")
                                    .child(task.collected.clone()),
                            )
                            .child(div().text_xs().text_color(rgb(0x999999)).child("已采集")),
                    )
                    .child(
                        div()
                            .text_center()
                            .child(
                                div()
                                    .font_weight(FontWeight::BOLD)
                                    .text_sm()
                                    .font_family("IBM Plex Mono")
                                    .child(task.success_rate.clone()),
                            )
                            .child(div().text_xs().text_color(rgb(0x999999)).child("成功率")),
                    ),
            )
            .child(
                // Actions
                div()
                    .flex()
                    .gap(px(8.0))
                    .child(self.render_action_button("✏️".into()))
                    .child(self.render_action_button("📄".into()))
                    .child(self.render_action_button("🗑️".into())),
            )
    }

    fn render_action_button(&self, icon: SharedString) -> Div {
        div()
            .w(px(32.0))
            .h(px(32.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_sm()
            .text_color(rgb(0x999999))
            .hover(|style| style.bg(rgb(0xffffff)).text_color(rgb(0x111111)))
            .cursor_pointer()
            .child(icon)
    }

    /// 渲染详情面板
    fn render_detail_panel(&self, _cx: &mut Context<Self>) -> Div {
        div()
            .absolute()
            .right(px(0.0))
            .top(px(0.0))
            .w(px(480.0))
            .h_full()
            .bg(rgb(0xffffff))
            .border_l_1()
            .border_color(rgb(0xe8e8e8))
            .shadow_lg()
            .flex()
            .flex_col()
            .child(
                // Header
                div()
                    .flex()
                    .items_start()
                    .justify_between()
                    .p(px(24.0))
                    .border_b_1()
                    .border_color(rgb(0xe8e8e8))
                    .child(
                        div()
                            .child(
                                div()
                                    .text_lg()
                                    .font_weight(FontWeight::BOLD)
                                    .child("任务配置"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0x999999))
                                    .child("ID: task_202403041200"),
                            ),
                    )
                    .child(
                        div()
                            .w(px(32.0))
                            .h(px(32.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded_sm()
                            .text_color(rgb(0x666666))
                            .cursor_pointer()
                            .child("✕"),
                    ),
            )
            .child(
                // Content
                div()
                    .flex_1()
                    .p(px(24.0))
                    .child(self.render_form_section(
                        "基本信息",
                        vec![
                            ("任务名称", "电商商品价格采集", "输入任务名称"),
                            ("环境标签", "PROD - 生产环境", ""),
                        ],
                    ))
                    .child(self.render_form_section(
                        "数据源配置",
                        vec![
                            (
                                "任务清单接口 URL",
                                "https://api.example.com/tasks",
                                "https://",
                            ),
                            ("请求方法", "GET", ""),
                        ],
                    )),
            )
            .child(
                // Footer
                div()
                    .flex()
                    .gap(px(12.0))
                    .p(px(24.0))
                    .border_t_1()
                    .border_color(rgb(0xe8e8e8))
                    .child(
                        div()
                            .flex_1()
                            .px(px(16.0))
                            .py(px(10.0))
                            .rounded_sm()
                            .border_1()
                            .border_color(rgb(0x0066ff))
                            .text_color(rgb(0x0066ff))
                            .text_sm()
                            .cursor_pointer()
                            .child("取消"),
                    )
                    .child(
                        div()
                            .flex_1()
                            .px(px(16.0))
                            .py(px(10.0))
                            .rounded_sm()
                            .bg(rgb(0x0066ff))
                            .text_color(rgb(0xffffff))
                            .text_sm()
                            .cursor_pointer()
                            .child("保存配置"),
                    ),
            )
    }

    fn render_form_section(&self, title: &str, fields: Vec<(&str, &str, &str)>) -> Div {
        div()
            .mb(px(28.0))
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x999999))
                    .mb(px(16.0))
                    .child(title.to_string()),
            )
            .children(fields.into_iter().map(|(label, value, placeholder)| {
                let val = if value.is_empty() {
                    placeholder.to_string()
                } else {
                    value.to_string()
                };
                div()
                    .mb(px(20.0))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .mb(px(8.0))
                            .child(label.to_string()),
                    )
                    .child(
                        div()
                            .w_full()
                            .px(px(14.0))
                            .py(px(12.0))
                            .border_1()
                            .border_color(rgb(0xe8e8e8))
                            .rounded_sm()
                            .text_sm()
                            .bg(rgb(0xf5f5f5))
                            .text_color(rgb(0x111111))
                            .child(val),
                    )
            }))
    }
}
