mod app;
pub mod components;
pub mod macros;
pub mod traits;
pub mod utils;

use crate::app::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let app = App::new();

    let json = serde_json::to_string_pretty(&app).unwrap();

    log!("{}", json);

    dominator::append_dom(&dominator::body(), App::render(app));

    Ok(())
}
