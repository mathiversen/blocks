use dominator::{class, html, Dom};
use futures_signals::signal::Mutable;
use once_cell::sync::Lazy;
use std::sync::Arc;

use crate::traits::Component;

#[derive(Debug)]
pub struct Title(pub Mutable<String>);

impl Default for Title {
    fn default() -> Self {
        Self(Mutable::new("Page title".into()))
    }
}

impl Component for Title {
    fn render(c: Arc<Self>) -> Dom {
        static STYLES: Lazy<String> = Lazy::new(|| {
            class! {
                .style("font-size", "1rem")
            }
        });

        html!("h1", {
            .class(&*STYLES)
            .text_signal(c.0.signal_cloned())
        })
    }
}
