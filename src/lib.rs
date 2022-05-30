mod app;
pub mod components;
pub mod macros;
pub mod sections;
pub mod traits;
pub mod utils;

use crate::{
    app::App,
    utils::dom::{storage, WebStore},
};
use wasm_bindgen::prelude::*;

const ADDRESS: &str = "http://localhost:1337";
const STATE_KEY: &str = "state";

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let storage = storage(WebStore::Local)?;

    let app = if let Some(data) = storage.get_item(STATE_KEY)? {
        console_log!("loading state from local storage");
        serde_json::from_str(&data).expect("valid state data")
    } else {
        console_log!("new state");
        App::new()
    };

    let json = serde_json::to_string_pretty(&app).unwrap();
    console_log!("{}", json);

    dominator::append_dom(&dominator::body(), App::render(app));

    Ok(())
}
