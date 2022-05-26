use futures_signals::signal::{Mutable, Signal, SignalExt};
use web_sys::Url;

pub fn url(href: &str) -> Url {
    Url::new(&format!("http://localhost:1337{}", href)).unwrap()
}

pub fn url_signal_string(url: Mutable<Url>) -> impl Signal<Item = String> {
    url.signal_cloned().map(|x| format!("{}", x.to_string()))
}

pub fn url_signal_string_svg(url: Mutable<Url>) -> impl Signal<Item = String> {
    url_signal_string(url).map(|x| x.replace("http://localhost:1337", ""))
}
