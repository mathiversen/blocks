use std::{pin::Pin, sync::Arc};

use crate::{components::icon::Icon, prelude::*};
use dominator::{class, html, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct LinkArgs {
    pub href: String,
    pub text: String,
    pub icon: Option<Arc<Icon>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub href: Mutable<String>,
    pub text: Mutable<String>,
    pub icon: Option<Arc<Icon>>,
    pub visible: Mutable<bool>,
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

    fn render<F>(c: Arc<Self>, _on_event: F) -> Dom
    where
        F: FnMut(Event) + 'static,
    {
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
            .attr_signal("href", c.href.signal_cloned())
            .apply_if(c.icon.is_some(), |dom| {
                dom.child(Icon::render(c.icon.clone().unwrap_ext(), move |_event| {}))
            })
            .child(html!("small", {
                .text_signal(c.text.signal_cloned())
            }))
        })
    }
}
