pub mod task;
pub mod welcome;

use gpui::{px, AnyView, App, AppContext, Entity, Hsla, Pixels, Render, Window};
use gpui_component::dock::PanelControl;
pub use task::TaskManager;
pub use task::TaskManager as TaskManagerView;
pub use welcome::Welcome;

pub(crate) fn init(cx: &mut App) {
    // input_story::init(cx);
    // rating_story::init(cx);
    // number_input_story::init(cx);
    // textarea_story::init(cx);
    // select_story::init(cx);
    // popover_story::init(cx);
    // menu_story::init(cx);
    // tooltip_story::init(cx);
    // otp_input_story::init(cx);
    // tree_story::init(cx);
}

pub trait ViewTrait: Render + Sized {
    fn klass() -> &'static str {
        std::any::type_name::<Self>().split("::").last().unwrap()
    }

    fn title() -> &'static str;

    fn description() -> &'static str {
        ""
    }

    fn closable() -> bool {
        true
    }

    fn zoomable() -> Option<PanelControl> {
        Some(PanelControl::default())
    }

    fn title_bg() -> Option<Hsla> {
        None
    }

    fn paddings() -> Pixels {
        px(16.)
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render>;

    fn on_active(&mut self, active: bool, window: &mut Window, cx: &mut App) {
        let _ = active;
        let _ = window;
        let _ = cx;
    }

    fn on_active_any(view: AnyView, active: bool, window: &mut Window, cx: &mut App)
    where
        Self: 'static,
    {
        if let Some(story) = view.downcast::<Self>().ok() {
            cx.update_entity(&story, |story, cx| {
                story.on_active(active, window, cx);
            });
        }
    }
}
