use crate::{components::header::Header, traits::Component};
use dominator::{clone, events, html, with_node, Dom};
use futures_signals::signal_vec::SignalVecExt;
use std::sync::Arc;
use web_sys::HtmlInputElement;

#[derive(Debug)]
pub struct App {
    pub header: Arc<Header>,
}

impl App {
    pub fn new() -> Arc<Self> {
        Arc::new({
            Self {
                header: Arc::new(Header::default()),
            }
        })
    }

    pub fn render(app: Arc<Self>) -> Dom {
        html!("app-root", {
            .child(Header::render(app.header.clone()))
            .child(html!("input" => HtmlInputElement, {
                .prop_signal("value", app.header.title.0.signal_cloned())
                .with_node!(element => {
                    .event(clone!(app => move |_: events::Input| {
                        app.header.title.0.set(element.value());
                    }))
                })
            }))
            .children_signal_vec(app.header.links.signal_vec_cloned().map(|link| {
                html!("input" => HtmlInputElement, {
                    .prop_signal("value", link.text.signal_cloned())
                    .with_node!(element => {
                        .event(clone!(element => move |_: events::Input| {
                            link.text.set(element.value());
                        }))
                    })
                })
            }))
        })
    }
}
