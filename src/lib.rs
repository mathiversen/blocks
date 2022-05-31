mod app;
pub mod components;
pub mod macros;
pub mod sections;
mod traits;
mod utils;

pub mod prelude {
    use std::pin::Pin;

    use futures_signals::signal::Signal;

    pub use super::traits::blocks::Component;
    pub use super::traits::blocks::Section;
    pub use super::traits::unwrap::UnwrapExt;
    pub use super::utils::*;

    pub type SignalReturn<A> = Pin<Box<dyn Signal<Item = A>>>;
}

use crate::app::App;
use wasm_bindgen::prelude::*;

const ADDRESS: &str = "http://localhost:1337";

#[wasm_bindgen(start)]
pub async fn main() {
    console_error_panic_hook::set_once();
    dominator::append_dom(&dominator::body(), App::render(App::new()));
}
