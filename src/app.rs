use dominator::{clone, events, html, with_node, Dom};
use futures_signals::signal::Mutable;
use std::sync::Arc;
use web_sys::HtmlInputElement;

#[derive(Debug, Clone)]
pub struct App {
    pub title: Mutable<String>,
}

impl App {
    pub fn new() -> Arc<Self> {
        Arc::new({
            Self {
                title: Mutable::new("App title".to_string()),
            }
        })
    }

    pub fn render(app: Arc<Self>) -> Dom {
        html!("app-root", {
            .children(&mut [
                html!("h1", {
                    .text_signal(app.title.signal_cloned())
                }),
                html!("input" => HtmlInputElement, {
                    .prop_signal("value", app.title.signal_cloned())
                    .with_node!(element => {
                        .event(clone!(app => move |_: events::Input| {
                            app.title.set(element.value());
                        }))
                    })
                })
            ])
        })
    }
}
