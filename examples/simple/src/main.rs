use block_components::prelude::*;
use dominator::html;
use std::sync::Arc;

pub fn main() {
    console_error_panic_hook::set_once();

    let text = Arc::new(Text::new(("h1".into(), "Hello world!".into())));

    dominator::append_dom(
        &dominator::body(),
        html! { "app-root", {
            .child(Text::render(text.clone(), |_| {}))
        }},
    );
}
