use dominator::{class, clone, events, html, Dom};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{components::icon::Icon, console_err, console_log, prelude::*};

#[derive(Debug)]
pub struct BannerArgs {
    pub text: String,
    pub href: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Banner {
    pub text: Mutable<String>,
    pub href: Mutable<String>,
    pub close_icon: Arc<Icon>,
    pub visible: Mutable<bool>,
}

impl Section for Banner {}

impl Component for Banner {
    type Argument = BannerArgs;

    fn new(args: Self::Argument) -> Self {
        Self {
            text: Mutable::new(args.text),
            href: Mutable::new(args.href),
            close_icon: Arc::new(Icon::new("x-square".into())),
            visible: Mutable::new(true),
        }
    }

    fn is_visible(&self) -> SignalReturn<bool> {
        self.visible.signal().boxed()
    }
    fn render<F>(c: Arc<Self>, _on_event: F) -> Dom
    where
        F: FnMut(Event) + 'static,
    {
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
            .attr("data-name", &c.get_component_name())
            .attr_signal("visible", c.is_visible().map(|x| x.to_string()))
            .visible_signal(c.is_visible())
            .after_inserted(clone!(c => move |_| {
                if let Some(data) = c.load_from_storage() {
                    c.text.set_neq(data.text.get_cloned());
                    c.href.set_neq(data.href.get_cloned());
                    c.visible.set_neq(data.visible.get());
                }
            }))
            .future(map_ref! {
                let _ = c.text.signal_cloned(),
                let _ = c.visible.signal_cloned(),
                let _ = c.href.signal_cloned() =>
                ()
            }.for_each(clone!(c => move |_| {
                if let Err(e) = c.save_to_storage() {
                    console_err!("{:?}", e);
                }
                async {}
            })))
            .child(html!("p", {
                .text_signal(c.text.signal_cloned())
                .child(html!("a", {
                    .class(&*A_STYLES)
                    .attr_signal("href", c.href.signal_cloned())
                    .text("Learn more.")
                }))
            }))
            .child(html!("button", {
                .class(&*BUTTON_STYLES)
                .child(Icon::render(c.close_icon.clone(), |event| {
                    console_log!("Banner got: {:?}", event)
                }))
                .event(clone!(c => move |_: events::Click| {
                    c.visible.set_neq(false)
                }))
            }))
        })
    }
}
