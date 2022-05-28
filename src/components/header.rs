use crate::{
    components::{link::Link, text::Text},
    traits::Component,
};
use dominator::{class, html, Dom};
use futures_signals::{
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{pin::Pin, sync::Arc};

#[derive(Debug)]
pub struct HeaderArgs {
    pub title: (String, String),
    pub links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub title: Arc<Text>,
    pub links: MutableVec<Arc<Link>>,
    pub visible: Mutable<bool>,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            title: Arc::new(Text::default()),
            links: MutableVec::new_with_values(
                (0..5)
                    .map(|_| Arc::new(Link::default()))
                    .collect::<Vec<_>>(),
            ),
            visible: Mutable::new(true),
        }
    }
}

impl Component for Header {
    type Argument = HeaderArgs;
    fn new(args: Self::Argument) -> Self {
        Self {
            title: Arc::new(Text::new(args.title)),
            links: MutableVec::new_with_values(
                args.links
                    .into_iter()
                    .map(|l| Arc::new(l))
                    .collect::<Vec<_>>(),
            ),
            ..Default::default()
        }
    }
    fn is_visible(&self) -> Pin<Box<dyn Signal<Item = bool>>> {
        self.visible.signal().boxed()
    }
    fn render(c: Arc<Self>) -> Dom {
        static HEADER_STYLES: Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "grid")
                .style("grid-template-columns", "1fr auto")
                .style("align-items", "center")
                .style("gap", "1rem")
                .style("padding", "0.5rem 1rem")
                .style("background", "lightgrey")
            }
        });

        static LINK_STYLES: Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "grid")
                .style("list-style", "none")
                .style("grid-auto-flow", "column")
                .style("gap", "1rem")
                .style("padding", "unset")
                .style("margin", "unset")
            }
        });

        html!("header", {
            .class(&*HEADER_STYLES)
            .child(Text::render(c.title.clone()))
            .child(html!("ol", {
                .class(&*LINK_STYLES)
                .children_signal_vec(c.links.signal_vec_cloned().map(|link| {
                    html!("li", {
                        .child(Link::render(link.clone()))
                    })
                }))
            }))
        })
    }
}
