use block_components::prelude::*;
use dominator::{clone, html, routing, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use std::sync::Arc;

pub struct App {
    icon: Arc<Icon>,
    url: Mutable<String>,
}

impl App {
    pub fn new() -> Arc<Self> {
        let url = routing::url().get_cloned();

        Arc::new(Self {
            icon: Arc::new(Icon::new("activity".into())),
            url: Mutable::new(url),
        })
    }

    pub fn render(state: Arc<Self>) -> Dom {
        html! { "app-root", {
            .future(routing::url()
                .signal_cloned()
                .for_each(clone!(state => move |url| {
                    state.url.set_neq(url.to_string());
                    async {}
                }))
            )
            .text_signal(state.url.signal_cloned())
            .child(Icon::render(state.icon.clone(), |_| {}))
        }}
    }
}
