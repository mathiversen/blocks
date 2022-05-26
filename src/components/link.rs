use std::sync::Arc;

use crate::{
    components::icon::Icon,
    helpers::{url, url_signal_string},
    traits::Component,
};
use dominator::{class, html, Dom};
use futures_signals::signal::Mutable;
use once_cell::sync::Lazy;
use web_sys::Url;

#[derive(Debug)]
pub struct Link {
    pub href: Mutable<Url>,
    pub text: Mutable<String>,
    pub icon: Arc<Icon>,
}

impl Default for Link {
    fn default() -> Self {
        Self {
            href: Mutable::new(url("/")),
            text: Mutable::new("Text".into()),
            icon: Arc::new(Icon::new("box")),
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

        html!("a", {
            .class(&*A_STYLES)
            .attr_signal("href", url_signal_string(c.href.clone()))
            .child(Icon::render(c.icon.clone()))
            .child(html!("small", {
                .text_signal(c.text.signal_cloned())
            }))
        })
    }
}
