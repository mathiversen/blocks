use wasm_bindgen::prelude::*;
mod app;
use app::App;

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let app = App::new();

    dominator::append_dom(&dominator::body(), App::render(app));

    Ok(())
}
