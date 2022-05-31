use crate::prelude::*;
use web_sys::{self, Document, Storage, Window};

pub fn window() -> Window {
    web_sys::window().unwrap_ext()
}

pub fn document() -> Document {
    window().document().unwrap_ext()
}

pub fn local_storage() -> Storage {
    window().local_storage().unwrap_ext().unwrap_ext()
}

pub fn session_storage() -> Storage {
    window().session_storage().unwrap_ext().unwrap_ext()
}
