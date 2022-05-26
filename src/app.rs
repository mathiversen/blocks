use crate::{
    components::{banner::Banner, header::Header},
    traits::Component,
};
use dominator::{class, html, Dom};
use once_cell::sync::Lazy;
use std::sync::Arc;

#[derive(Debug)]
pub struct App {
    pub banner: Arc<Banner>,
    pub header: Arc<Header>,
}

impl App {
    pub fn new() -> Arc<Self> {
        Arc::new({
            Self {
                banner: Arc::new(Banner::default()),
                header: Arc::new(Header::default()),
            }
        })
    }

    pub fn render(app: Arc<Self>) -> Dom {
        static ROOT_STYLES: Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "grid")
                .style("grid-template-rows", "auto auto 1fr auto")
                .style("height", "100svh")
            }
        });

        html!("app-root", {
            .class(&*ROOT_STYLES)
            .child(Banner::render(app.banner.clone()))
            .child(Header::render(app.header.clone()))
            .child(html!("main", {
                .text("hello world")
            }))
            .child(html!("footer", {
                .text("footer")
            }))
            // .child(html!("input" => HtmlInputElement, {
            //     .prop_signal("value", app.header.title.0.signal_cloned())
            //     .with_node!(element => {
            //         .event(clone!(app => move |_: events::Input| {
            //             app.header.title.0.set(element.value());
            //         }))
            //     })
            // }))
            // .children_signal_vec(app.header.links.signal_vec_cloned().map(|link| {
            //     html!("input" => HtmlInputElement, {
            //         .prop_signal("value", link.text.signal_cloned())
            //         .with_node!(element => {
            //             .event(clone!(element => move |_: events::Input| {
            //                 link.text.set(element.value());
            //             }))
            //         })
            //     })
            // }))
        })
    }
}
