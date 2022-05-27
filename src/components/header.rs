use crate::{
    components::{link::Link, title::Title},
    traits::Component,
};
use dominator::{class, html, Dom};
use futures_signals::{
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use once_cell::sync::Lazy;
use std::{pin::Pin, sync::Arc};

#[derive(Debug)]
pub struct HeaderArgs {
    title: String,
    links: Vec<Link>,
}

#[derive(Debug)]
pub struct Header {
    pub title: Arc<Title>,
    pub links: MutableVec<Arc<Link>>,
    pub visible: Mutable<bool>,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            title: Arc::new(Title::default()),
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
            title: Mutable::new(args.title),
            links: MutableVec::new_with_values(
                args.links.iter().map(|l| Arc::new(l)).collect::<Vec<_>>(),
            ),
            visible: (),
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
            .child(Title::render(c.title.clone()))
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
