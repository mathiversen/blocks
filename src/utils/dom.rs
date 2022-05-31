use web_sys::{self, Document, Storage, Window};

pub fn window() -> Window {
    web_sys::window().unwrap()
}

pub fn document() -> Document {
    window().document().unwrap()
}

pub fn local_storage() -> Storage {
    window().local_storage().unwrap().unwrap()
}

pub fn session_storage() -> Storage {
    window().session_storage().unwrap().unwrap()
}
