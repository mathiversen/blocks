use crate::{
    components::{banner::Banner, header::Header},
    traits::Component,
};
use dominator::{class, html, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use once_cell::sync::Lazy;
use std::sync::Arc;

#[derive(Debug)]
pub struct App {
    pub banner: Arc<Banner>,
    pub header: Arc<Header>,
    pub footer: Arc<Mutable<Option<String>>>,
}

impl App {
    pub fn new() -> Arc<Self> {
        Arc::new({
            Self {
                banner: Arc::new(Banner::default()),
                header: Arc::new(Header::default()),
                footer: Arc::new(Mutable::new(Some("Iversen © 2022".into()))),
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

        static MAIN_STYLES: Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "grid")
                .style("block-size", "100%")
                .style("grid-auto-flow", "column")
                .style("grid-auto-columns", "100%")
                .style("overflow", "auto hidden")
                .style("background", "var(--color-white)")
                .style("scroll-snap-type", "x mandatory")
            }
        });

        static SECTION_STYLES: Lazy<String> = Lazy::new(|| {
            class! {
                .style("overflow-y", "auto")
                // .style("scroll-snap-type", "y proximity")
                .style("scroll-padding", "1rem")
                .style("scroll-snap-align", "start")
            }
        });

        html!("app-root", {
            .class(&*ROOT_STYLES)
            .style_signal("grid-template-rows", app.banner.clone().visible.signal_ref(|bool| {
                if *bool {
                    "auto auto 1fr auto"
                } else {
                    "auto 1fr auto"
                }
            }))
            .child(Banner::render(app.banner.clone()))
            .child(Header::render(app.header.clone()))
            .child(html!("main", {
                .class(&*MAIN_STYLES)
                .children((1..=5).map(|x| {
                    html!("section", {
                        .class(&*SECTION_STYLES)
                        .text(&format!("section {}", &x))
                    })
                }))
            }))
            .child(html!("footer", {
                .visible_signal(app.footer.signal_ref(|x| x.is_some()).dedupe())
                .style("text-align", "center")
                .child(html!("small", {
                    .text_signal(app.footer.signal_ref(|x| x.clone().unwrap()))
                }))
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
