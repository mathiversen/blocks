use std::sync::Arc;

use crate::traits::Component;
use dominator::{class, html, Dom};
use futures_signals::signal::Mutable;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct Link {
    pub href: Mutable<String>, // TODO: Use url
    pub text: Mutable<String>,
    pub icon_url: Mutable<String>,
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

        static IMG_STYLES: Lazy<String> = Lazy::new(|| {
            class! {
                .style("border-radius", "0.5rem")
            }
        });

        html!("a", {
            .class(&*A_STYLES)
            .attr_signal("href", c.href.signal_cloned())
            .child(html!("img", {
                .class(&*IMG_STYLES)
                .attr_signal("src", c.icon_url.signal_cloned())
                .attr("alt", "")
            }))
            .child(html!("small", {
                .text_signal(c.text.signal_cloned())
            }))
        })
    }
}

impl Default for Link {
    fn default() -> Self {
        Self {
            href: Mutable::new("/".into()),
            text: Mutable::new("Text".into()),
            icon_url: Mutable::new("http://www.placecage.com/24/24".into()),
        }
    }
}
