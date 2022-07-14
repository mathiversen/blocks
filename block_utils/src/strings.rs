use crate::traits::*;

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
    let last = std::any::type_name::<A>().split("::").last().unwrap_ext();
    to_snake_case(last)
}
