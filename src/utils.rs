use futures_signals::signal::{Mutable, Signal, SignalExt};
use serde::Serializer;
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

// https://github.com/serde-rs/serde/blob/master/serde_derive/src/internals/case.rs
pub fn to_snake_case(text: &str) -> String {
    let mut snake = String::new();
    for (i, ch) in text.char_indices() {
        if i > 0 && ch.is_uppercase() {
            snake.push('_');
        }
        snake.push(ch.to_ascii_lowercase());
    }
    snake
}

pub fn get_struct_name<A>() -> String {
    let last = std::any::type_name::<A>().split("::").last().unwrap();
    to_snake_case(last)
}

pub fn serialize_url<S>(url: &Mutable<Url>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let url: String = url.read_only().get_cloned().to_string().into();
    s.serialize_str(&url)
}
