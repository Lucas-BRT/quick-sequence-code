use wasm_bindgen::prelude::*;
use web_sys::{Document, window};

pub fn get_document() -> Result<Document, JsValue> {
    let window = window().ok_or("No global `window` exists")?;
    window
        .document()
        .ok_or("Should have a document on window".into())
}
