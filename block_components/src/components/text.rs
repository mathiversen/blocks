use dominator::{class, html, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    tag: String,
    text: Mutable<String>,
    visible: Mutable<bool>,
}

impl Component for Text {
    type Argument = (String, String);

    fn new(args: Self::Argument) -> Self {
        Self {
            tag: args.0,
            text: Mutable::new(args.1),
            visible: Mutable::new(true),
        }
    }

    fn is_visible(&self) -> SignalReturn<bool> {
        self.visible.signal().boxed()
    }

    fn render<F>(c: Arc<Self>, _on_event: F) -> Dom
    where
        F: FnMut(Event) + 'static,
    {
        static STYLES: Lazy<String> = Lazy::new(|| {
            class! {
                .style("font-size", "1rem")
                .style("font-weight", "normal")
            }
        });

        html!(&c.tag, {
            .class(&*STYLES)
            .visible_signal(c.is_visible())
            .text_signal(c.text.signal_cloned())
        })
    }
}
