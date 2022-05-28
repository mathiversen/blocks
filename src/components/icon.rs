use std::{pin::Pin, sync::Arc};

use crate::{
    traits::Component,
    utils::{serialize_url, url, url_signal_string_svg},
};
use dominator::{class, svg, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use once_cell::sync::Lazy;
use serde::Serialize;
use web_sys::Url;

#[derive(Debug, Serialize)]
pub struct Icon {
    #[serde(serialize_with = "serialize_url")]
    pub href: Mutable<Url>,
    pub visible: Mutable<bool>,
}

impl Default for Icon {
    fn default() -> Self {
        Self {
            href: Mutable::new(url("/static/svg/icons.svg#box")),
            visible: Mutable::new(true),
        }
    }
}

impl Component for Icon {
    type Argument = String;

    fn new(args: Self::Argument) -> Self {
        Self {
            href: Mutable::new(url(&format!("/static/svg/icons.svg#{}", args))),
            ..Default::default()
        }
    }
    fn is_visible(&self) -> Pin<Box<dyn Signal<Item = bool>>> {
        self.visible.signal().boxed()
    }
    fn render(c: Arc<Self>) -> Dom {
        static STYLES: Lazy<String> = Lazy::new(|| {
            class! {
                .style("width", "24px")
                .style("height", "24px")
                .style("stroke", "var(--color-black)")
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
                .attr_signal("href", url_signal_string_svg(c.href.clone()))
            }))
        })
    }
}
