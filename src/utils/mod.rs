pub mod dom;

use crate::{console_err, ADDRESS};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{
    fmt,
    io::{Error, ErrorKind},
    ops::Deref,
};
use web_sys::{self, Url as WebUrl};

#[derive(Debug, Clone, PartialEq)]
pub struct Url(WebUrl);

impl Deref for Url {
    type Target = WebUrl;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for Url {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match WebUrl::new(value) {
            Ok(x) => Ok(Url(x)),
            Err(x) => Err(Error::new(ErrorKind::Other, format!("{:?}", x))),
        }
    }
}

impl Serialize for Url {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let url: String = self.0.to_string().into();
        serializer.serialize_str(&url)
    }
}

impl<'de> Deserialize<'de> for Url {
    fn deserialize<D>(deserializer: D) -> Result<Url, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StrVisitor;

        impl<'de> Visitor<'de> for StrVisitor {
            type Value = Url;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Url::try_from(value).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(StrVisitor)
    }
}

// TODO: Shuld return Result, so that we can catch errors
pub fn url(url: &str) -> Url {
    let url = format!("{}{}", ADDRESS, url);
    match Url::try_from(url.as_str()) {
        Ok(url) => url,
        Err(_) => {
            console_err!("{}", format!("Invalid url: {}", url));
            Url(WebUrl::new(&format!("{}/", ADDRESS)).expect("Invalid url"))
        }
    }
}

pub fn url_signal_string(url: Mutable<Url>) -> impl Signal<Item = String> {
    url.signal_cloned().map(|x| format!("{}", x.to_string()))
}

pub fn url_signal_string_svg(url: Mutable<Url>) -> impl Signal<Item = String> {
    url_signal_string(url).map(|x| x.replace(ADDRESS, ""))
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
