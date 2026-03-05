use dut_box::Welcome;
use gpui::{prelude::*, *};
use wasm_bindgen::prelude::*;

/// 与 web 前端兼容的入口名
#[wasm_bindgen]
pub fn init_app(_canvas_id: String) -> Result<(), JsValue> {
    run_app()
}

fn run_app() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    console_log::init_with_level(log::Level::Info).expect("Failed to initialize logger");
    tracing_wasm::set_as_global_default();

    #[cfg(not(target_family = "wasm"))]
    let app = gpui_platform::application();
    #[cfg(target_family = "wasm")]
    let app = gpui_platform::single_threaded_web();

    app.run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| Welcome::new()))
            .expect("Failed to open window");
        cx.activate(true);
    });

    Ok(())
}
