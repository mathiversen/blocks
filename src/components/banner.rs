use dominator::{class, html, Dom};
use futures_signals::signal::Mutable;
use once_cell::sync::Lazy;
use std::sync::Arc;
use web_sys::Url;

use crate::{helpers::url_signal_to_str, traits::Component};

#[derive(Debug)]
pub struct Banner {
    text: Mutable<String>,
    href: Mutable<Url>,
}

impl Default for Banner {
    fn default() -> Self {
        Self {
            text: Mutable::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. ".into()),
            href: Mutable::new(Url::new("localhost:1337/").expect("Valid url")),
        }
    }
}

impl Component for Banner {
    fn render(c: Arc<Self>) -> Dom {
        static STYLES: Lazy<String> = Lazy::new(|| {
            class! {
                .style("padding", "1rem")
                .style("text-align", "center")
            }
        });

        static A_STYLES: Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "inline-flex")
                .style("flex-direction", "row")
            }
        });

        html!("aside", {
            .class(&*STYLES)
            .child(html!("p", {
                .text_signal(c.text.signal_cloned())
                .child(html!("a", {
                    .class(&*A_STYLES)
                    .attr_signal("href", url_signal_to_str(c.href.clone()))
                    .text("Learn more.")
                }))
            }))
        })
    }
}
