use std::{pin::Pin, sync::Arc};

use crate::{
    components::icon::Icon,
    traits::Component,
    utils::{url, url_signal_string, Url},
};
use dominator::{class, html, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct LinkArgs {
    pub href: Url,
    pub text: String,
    pub icon: Option<Arc<Icon>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub href: Mutable<Url>,
    pub text: Mutable<String>,
    pub icon: Option<Arc<Icon>>,
    pub visible: Mutable<bool>,
}

impl Default for Link {
    fn default() -> Self {
        Self {
            href: Mutable::new(url("/")),
            text: Mutable::new("Text".into()),
            icon: None,
            visible: Mutable::new(true),
        }
    }
}

impl Component for Link {
    type Argument = LinkArgs;

    fn new(args: Self::Argument) -> Self {
        Self {
            href: Mutable::new(args.href),
            text: Mutable::new(args.text),
            icon: args.icon,
            visible: Mutable::new(true),
        }
    }

    fn is_visible(&self) -> Pin<Box<dyn Signal<Item = bool>>> {
        self.visible.signal().boxed()
    }
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
            .visible_signal(c.is_visible())
            .class(&*A_STYLES)
            .attr_signal("href", url_signal_string(c.href.clone()))
            .apply_if(c.icon.is_some(), |dom| {
                dom.child(Icon::render(c.icon.clone().unwrap()))
            })
            .child(html!("small", {
                .text_signal(c.text.signal_cloned())
            }))
        })
    }
}
