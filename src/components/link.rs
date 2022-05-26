use std::sync::Arc;

use crate::traits::Component;
use dominator::{class, html, svg, Dom};
use futures_signals::signal::Mutable;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct Link {
    pub href: Mutable<String>, // TODO: Use url
    pub text: Mutable<String>,
    pub icon_url: Mutable<String>,
}

impl Default for Link {
    fn default() -> Self {
        Self {
            href: Mutable::new("/".into()),
            text: Mutable::new("Text".into()),
            icon_url: Mutable::new("static/svg/icons.svg#activity".into()),
        }
    }
}

impl Component for Link {
    fn render(c: Arc<Self>) -> Dom {
        static A_STYLES: Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "grid")
                .style("text-align", "center")
                .style("justify-items", "center")
                .style("gap", "0.1rem")
            }
        });

        static SVG_STYLES: Lazy<String> = Lazy::new(|| {
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

        html!("a", {
            .class(&*A_STYLES)
            .attr_signal("href", c.href.signal_cloned())
            .child(svg!("svg", {
                .attr("xmlns", "http://www.w3.org/2000/svg")
                .class(&*SVG_STYLES)
                .child(svg!("use", {
                    .attr_signal("href", c.icon_url.signal_cloned())
                }))
            }))
            .child(html!("small", {
                .text_signal(c.text.signal_cloned())
            }))
        })
    }
}
