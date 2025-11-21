use crate::APPLICATION_NAME;
use crate::events::setup_file_selector;
use crate::utils::get_document;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, console};

pub fn update_initialization_message(success: Option<bool>) -> Result<(), JsValue> {
    let document = get_document()?;
    let msg_element = document
        .get_element_by_id("msg")
        .ok_or("Message element not found")?
        .dyn_into::<HtmlElement>()?;

    if success.unwrap_or(true) {
        let msg_content = APPLICATION_NAME;
        msg_element.set_text_content(Some(msg_content));
    } else {
        msg_element.set_text_content(Some("INITIALIZATION FAILED"));
        msg_element
            .style()
            .set_property("color", "var(--accent-orange)")?;
    }

    msg_element.class_list().remove_1("loading")?;
    Ok(())
}

#[wasm_bindgen]
pub fn init_app() -> Result<(), JsValue> {
    if let Err(e) = setup_file_selector() {
        console::error_1(&format!("Failed to initialize application: {:?}", e).into());
        update_initialization_message(Some(false))?;
    } else {
        update_initialization_message(Some(true))?;
        console::log_1(&"Application initialized successfully".into());
    }
    Ok(())
}
