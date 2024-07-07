#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use gbmu::app::App;
#[cfg(not(target_arch = "wasm32"))]
use std::error::Error;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn Error>> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "GBMU",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )?;

    Ok(())
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok(); // TODO why .ok()

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let start_result = eframe::WebRunner::new()
            .start(
                "gbmu_canvas",
                web_options,
                // TODO web version does not work yet as the rom can't be read
                Box::new(|cc| Ok(Box::new(App::new(cc)))),
            )
            .await;
        let loading_text = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("loading_text"));
        match start_result {
            Ok(_) => {
                loading_text.map(|e| e.remove());
            }
            Err(e) => {
                loading_text.map(|e| {
                    e.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    )
                });
                panic!("failed to start eframe: {e:?}");
            }
        }
    });
}
