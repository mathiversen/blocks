use crate::prelude::*;
use dominator::{class, svg, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Icon {
    pub href: Mutable<String>,
    pub visible: Mutable<bool>,
}

impl Component for Icon {
    type Argument = String;

    fn new(name: Self::Argument) -> Self {
        Self {
            href: Mutable::new(format!("/static/svg/icons.svg#{}", name)),
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
                .style("width", "24px")
                .style("height", "24px")
                .style("stroke", "var(--color-black")
                .style("stroke-width", "2")
                .style("stroke-linecap", "round")
                .style("stroke-linejoin", "round")
                .style("fill", "var(--color-transparent)")
            }
        });

        svg!("svg", {
            .attr("xmlns", "http://www.w3.org/2000/svg")
            .visible_signal(c.is_visible())
            .class(&*STYLES)
            .child(svg!("use", {
                .attr_signal("href", c.href.signal_cloned())
            }))
        })
    }
}
