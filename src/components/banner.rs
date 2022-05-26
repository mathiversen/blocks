use dominator::{class, clone, events, html, Dom};
use futures_signals::signal::Mutable;
use once_cell::sync::Lazy;
use std::sync::Arc;
use web_sys::Url;

use crate::{helpers::url_signal_string, traits::Component};

use super::icon::Icon;

#[derive(Debug)]
pub struct Banner {
    pub text: Mutable<String>,
    pub href: Mutable<Url>,
    pub close_icon: Arc<Icon>,
    pub visible: Mutable<bool>,
}

impl Default for Banner {
    fn default() -> Self {
        Self {
            text: Mutable::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. ".into()),
            href: Mutable::new(Url::new("localhost:1337/").expect("Valid url")),
            close_icon: Arc::new(Icon::new("x")),
            visible: Mutable::new(true)
        }
    }
}

impl Component for Banner {
    fn render(c: Arc<Self>) -> Dom {
        static STYLES: Lazy<String> = Lazy::new(|| {
            class! {
                .style("position", "relative")
                .style("padding", "1rem calc(2rem + 24px) 1rem 1rem")
                .style("text-align", "center")
            }
        });

        static A_STYLES: Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "inline-flex")
                .style("flex-direction", "row")
            }
        });

        static BUTTON_STYLES: Lazy<String> = Lazy::new(|| {
            class! {
                .style("position", "absolute")
                .style("right", "1rem")
                .style("top", "1rem")
            }
        });

        html!("aside", {
            .class(&*STYLES)
            .visible_signal(c.visible.signal_cloned())
            .child(html!("p", {
                .text_signal(c.text.signal_cloned())
                .child(html!("a", {
                    .class(&*A_STYLES)
                    .attr_signal("href", url_signal_string(c.href.clone()))
                    .text("Learn more.")
                }))
            }))
            .child(html!("button", {
                .class(&*BUTTON_STYLES)
                .child(Icon::render(c.close_icon.clone()))
                .event(clone!(c => move |_: events::Click| {
                    c.visible.set_neq(false)
                }))
            }))
        })
    }
}