use dominator::{class, html, Dom};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    tag: Mutable<String>,
    text: Mutable<String>,
    visible: Mutable<bool>,
}

impl Component for Text {
    type Argument = (String, String);

    fn new(args: Self::Argument) -> Self {
        Self {
            tag: Mutable::new(args.0),
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
        static TEXT_WRAPPER: Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "contents")
            }
        });

        static TEXT: Lazy<String> = Lazy::new(|| {
            class! {
                .style("font-size", "1rem")
            }
        });

        // TODO: Remove wrapper
        html!("div", {
            .class(&*TEXT_WRAPPER)
            .visible_signal(c.is_visible())
            .child_signal(map_ref! {
                let text = c.text.signal_cloned(),
                let tag = c.tag.signal_cloned() =>
                (tag.clone(), text.clone())
            }.map(|(tag, text)| {
                Some(html!(&tag, {
                    .class(&*TEXT)
                    .text(&*text)
                }))
            }))
        })
    }
}
