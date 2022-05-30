use dominator::{class, html, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::traits::Component;

#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    text: Mutable<String>,
    tag: Mutable<String>,
    visible: Mutable<bool>,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            text: Mutable::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".into()),
            tag: Mutable::new("P".into()),
            visible: Mutable::new(true)
        }
    }
}

impl Component for Text {
    type Argument = (String, String);

    fn new(args: Self::Argument) -> Self {
        Self {
            text: Mutable::new(args.1),
            tag: Mutable::new(args.0),
            ..Default::default()
        }
    }

    fn is_visible(&self) -> crate::traits::SignalReturn<bool> {
        self.visible.signal().boxed()
    }
    fn render(c: Arc<Self>) -> Dom {
        static STYLES: Lazy<String> = Lazy::new(|| {
            class! {
                .style("font-size", "1rem")
            }
        });

        html!("p", {
            .class(&*STYLES)
            // .prop_signal("tagName", c.tag.signal_cloned())
            .text_signal(c.text.signal_cloned())
        })
    }
}
