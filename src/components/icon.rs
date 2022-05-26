use std::sync::Arc;

use crate::{
    helpers::{url, url_signal_string_svg},
    traits::Component,
};
use dominator::{class, svg, Dom};
use futures_signals::signal::Mutable;
use once_cell::sync::Lazy;
use web_sys::Url;

#[derive(Debug)]
pub struct Icon {
    pub href: Mutable<Url>,
}

impl Icon {
    pub fn new(name: &str) -> Self {
        Self {
            href: Mutable::new(url(&format!("/static/svg/icons.svg#{}", name))),
        }
    }
}

impl Component for Icon {
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
            .class(&*STYLES)
            .child(svg!("use", {
                .attr_signal("href", url_signal_string_svg(c.href.clone()))
            }))
        })
    }
}
