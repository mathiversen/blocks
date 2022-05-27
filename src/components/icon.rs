use std::{pin::Pin, sync::Arc};

use crate::{
    traits::Component,
    utils::{url, url_signal_string_svg},
};
use dominator::{class, svg, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use once_cell::sync::Lazy;
use web_sys::Url;

#[derive(Debug)]
pub struct Icon {
    pub href: Mutable<Url>,
    pub visible: Mutable<bool>,
}

// impl Icon {
//     pub fn new(name: &str) -> Self {
//         Self {
//             href: Mutable::new(url(&format!("/static/svg/icons.svg#{}", name))),
//             visible: Mutable::new(true),
//         }
//     }
// }

impl Component for Icon {
    type Argument = &'static str;

    fn new(args: Self::Argument) -> Self {
        Self {
            href: Mutable::new(url(&format!("/static/svg/icons.svg#{}", args))),
            visible: Mutable::new(true),
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
