use futures_signals::signal::{Mutable, Signal, SignalExt};
use web_sys::Url;

pub fn url(href: &str) -> Url {
    Url::new(&format!("http://localhost:1337{}", href)).unwrap()
}

pub fn url_signal_to_str(url: Mutable<Url>) -> impl Signal<Item = String> {
    url.signal_cloned().map(|x| format!("{}", x.to_string()))
}
