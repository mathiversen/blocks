mod app;
pub mod components;
pub mod macros;
pub mod sections;
pub mod traits;
pub mod utils;

use crate::app::App;
use wasm_bindgen::prelude::*;

const ADDRESS: &str = "http://localhost:1337";

#[wasm_bindgen(start)]
pub async fn main() {
    console_error_panic_hook::set_once();
    dominator::append_dom(&dominator::body(), App::render(App::new()));
}
