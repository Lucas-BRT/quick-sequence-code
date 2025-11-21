use crate::constants::{ANIMATION_DELAY_INCREMENT, DEFAULT_QR_CODE_SIZE, PERFORMANCE_THRESHOLD};
use crate::utils::get_document;
use js_sys::Array;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlCanvasElement, HtmlElement};

pub fn create_canvas(
    id: &str,
    width: Option<u32>,
    height: Option<u32>,
    animation_delay: Option<f64>,
) -> Result<HtmlCanvasElement, JsValue> {
    let document = get_document()?;
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;

    canvas.set_id(id);
    canvas.set_width(width.unwrap_or(DEFAULT_QR_CODE_SIZE));
    canvas.set_height(height.unwrap_or(DEFAULT_QR_CODE_SIZE));

    if let Some(delay) = animation_delay {
        let style = canvas.style();
        style.set_property("animation-delay", &format!("{}s", delay))?;
    }

    Ok(canvas)
}

pub fn create_status_message(text: &str, is_error: Option<bool>) -> Result<HtmlElement, JsValue> {
    let document = get_document()?;
    let div = document.create_element("div")?.dyn_into::<HtmlElement>()?;

    div.set_inner_html(&format!("<strong>{}</strong>", text));

    let color = if is_error.unwrap_or(false) {
        "#ff0000"
    } else {
        "var(--accent-orange)"
    };

    let style_text = format!(
        "width: 100%; text-align: center; color: {}; font-weight: bold; font-size: 1.1rem; margin-top: 2rem; text-transform: uppercase; letter-spacing: 0.1em;",
        color
    );

    div.style().set_css_text(&style_text);

    Ok(div)
}

pub fn create_loading_message() -> Result<HtmlElement, JsValue> {
    let document = get_document()?;
    let div = document.create_element("div")?.dyn_into::<HtmlElement>()?;

    div.set_text_content(Some("GENERATING QR CODES..."));

    let style_text = "text-align: center; color: var(--accent-orange); font-weight: bold; font-size: 1.2rem; margin: 2rem 0;";
    div.style().set_css_text(style_text);

    Ok(div)
}

pub fn create_error_message(message: Option<String>) -> Result<HtmlElement, JsValue> {
    let document = get_document()?;
    let div = document.create_element("div")?.dyn_into::<HtmlElement>()?;

    let error_text = message.unwrap_or_else(|| "ERROR GENERATING QR CODES".to_string());
    div.set_text_content(Some(&error_text));

    let style_text =
        "text-align: center; color: #ff0000; font-weight: bold; font-size: 1.2rem; margin: 2rem 0;";
    div.style().set_css_text(style_text);

    Ok(div)
}

pub fn clear_container(container_id: &str) -> Result<Element, JsValue> {
    let document = get_document()?;
    let container = document.get_element_by_id(container_id).ok_or_else(|| {
        JsValue::from_str(&format!("Container with id '{}' not found", container_id))
    })?;

    container.set_inner_html("");
    Ok(container)
}

pub fn append_elements_to_container(elements: &Array, container_id: &str) -> Result<(), JsValue> {
    let document = get_document()?;
    let container = document.get_element_by_id(container_id).ok_or_else(|| {
        JsValue::from_str(&format!("Container with id '{}' not found", container_id))
    })?;

    let fragment = document.create_document_fragment();

    for i in 0..elements.length() {
        if let Some(element) = elements.get(i).dyn_ref::<Element>() {
            fragment.append_child(element)?;
        }
    }

    container.append_child(&fragment)?;
    Ok(())
}

pub fn create_qrcode_elements_as_html(total_qrcodes: usize) -> String {
    let mut html_string = String::new();

    for i in 0..total_qrcodes {
        let animation_delay = i as f64 * ANIMATION_DELAY_INCREMENT;
        html_string.push_str(&format!(
            r#"<canvas id="qrcode-{}" width="{}" height="{}" style="animation-delay: {}s;"></canvas>"#,
            i, DEFAULT_QR_CODE_SIZE, DEFAULT_QR_CODE_SIZE, animation_delay
        ));
    }

    let plural_suffix = if total_qrcodes > 1 { "S" } else { "" };
    let message = format!("{} QR CODE{} GENERATED", total_qrcodes, plural_suffix);
    html_string.push_str(&format!(
        r#"<div style="width: 100%; text-align: center; color: var(--accent-orange); font-weight: bold; font-size: 1.1rem; margin-top: 2rem; text-transform: uppercase; letter-spacing: 0.1em;"><strong>{}</strong></div>"#,
        message
    ));

    html_string
}

pub fn append_elements_with_inner_html(
    total_qrcodes: usize,
    container_id: &str,
) -> Result<(), JsValue> {
    let document = get_document()?;
    let container = document.get_element_by_id(container_id).ok_or_else(|| {
        JsValue::from_str(&format!("Container with id '{}' not found", container_id))
    })?;

    let html_content = create_qrcode_elements_as_html(total_qrcodes);
    container.set_inner_html(&html_content);

    Ok(())
}

pub fn create_qrcode_elements(total_qrcodes: usize) -> Result<js_sys::Array, JsValue> {
    let elements = js_sys::Array::new();

    for i in 0..total_qrcodes {
        let canvas = create_canvas(
            &format!("qrcode-{}", i),
            Some(DEFAULT_QR_CODE_SIZE),
            Some(DEFAULT_QR_CODE_SIZE),
            Some(i as f64 * ANIMATION_DELAY_INCREMENT),
        )?;
        elements.push(&canvas);
    }

    let plural_suffix = if total_qrcodes > 1 { "S" } else { "" };
    let message = format!("{} QR CODE{} GENERATED", total_qrcodes, plural_suffix);
    let completion_message = create_status_message(&message, Some(false))?;
    elements.push(&completion_message);

    Ok(elements)
}

pub fn add_qrcode_elements_to_dom(total_qrcodes: usize, container_id: &str) -> Result<(), JsValue> {
    web_sys::console::log_1(
        &format!(
            "Creating {} QR codes using {} strategy",
            total_qrcodes,
            if total_qrcodes >= PERFORMANCE_THRESHOLD {
                "innerHTML"
            } else {
                "DocumentFragment"
            }
        )
        .into(),
    );

    if total_qrcodes >= PERFORMANCE_THRESHOLD {
        append_elements_with_inner_html(total_qrcodes, container_id)
    } else {
        let qrcode_elements = create_qrcode_elements(total_qrcodes)?;
        append_elements_to_container(&qrcode_elements, container_id)
    }
}

pub fn show_loading_state() -> Result<(), JsValue> {
    let container = clear_container("canvas-container")?;
    let loading_message = create_loading_message()?;
    container.append_child(&loading_message)?;
    Ok(())
}

pub fn show_error_state(message: Option<String>) -> Result<(), JsValue> {
    let container = clear_container("canvas-container")?;
    let error_message = create_error_message(message)?;
    container.append_child(&error_message)?;
    Ok(())
}
