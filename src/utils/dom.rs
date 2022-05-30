use wasm_bindgen::JsError;
use web_sys::{self, Document, Storage, Window};

#[derive(Debug)]
pub enum WebStore {
    Local,
    Session,
}

pub fn window() -> Result<Window, JsError> {
    web_sys::window().ok_or(JsError::new("No window"))
}

pub fn document() -> Result<Document, JsError> {
    window()?.document().ok_or(JsError::new("No document"))
}

pub fn storage(variant: WebStore) -> Result<Storage, JsError> {
    let res = match variant {
        WebStore::Local => window()?.local_storage(),
        WebStore::Session => window()?.session_storage(),
    };

    res.map_err(|_| JsError::new(&format!("Failed to get {:?}", variant)))?
        .ok_or(JsError::new(&format!(
            "Missing support for storage {:?}",
            variant
        )))
}
