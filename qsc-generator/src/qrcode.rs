use crate::DEFAULT_QR_CODE_VERSION;
use qrcode::{Color, QrCode};
use qrcode::{EcLevel, Version};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};

pub fn parse_into_qrcode(data: &[u8]) -> Result<Vec<bool>, String> {
    let qrcode =
        match QrCode::with_version(data, Version::Normal(DEFAULT_QR_CODE_VERSION), EcLevel::L) {
            Ok(code) => code,
            Err(e) => return Err(format!("Failed to create QR code: {}", e)),
        };

    let matrix = qrcode
        .into_colors()
        .into_iter()
        .map(|color| color == Color::Light)
        .collect::<Vec<bool>>();

    Ok(matrix)
}

pub fn render_qrcode_canvas(canvas_id: &str, data: &[u8]) -> Result<(), JsValue> {
    let window = window().ok_or("No global `window` exists")?;
    let document = window
        .document()
        .ok_or("Should have a document on window")?;

    let canvas = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| format!("Canvas with id '{}' not found", canvas_id))?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| "Element is not a canvas")?;

    let context = canvas
        .get_context("2d")
        .map_err(|_| "Failed to get 2d context")?
        .ok_or("No 2d context")?
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(|_| "Failed to cast to CanvasRenderingContext2d")?;

    let qr_code =
        parse_into_qrcode(data).map_err(|reason| format!("Failed to parse QR code: {}", reason))?;

    let modules = qr_code.clone();
    let size = qr_code.len().isqrt();

    let canvas_size = canvas.width();
    let module_size = canvas_size as f64 / size as f64;

    // Clear canvas
    context.clear_rect(0.0, 0.0, canvas_size as f64, canvas_size as f64);

    // Render QR code modules
    for y in 0..size {
        for x in 0..size {
            if modules[y * size + x] {
                context.set_fill_style_str("black");
            } else {
                context.set_fill_style_str("white");
            }

            context.fill_rect(
                x as f64 * module_size,
                y as f64 * module_size,
                module_size,
                module_size,
            );
        }
    }

    Ok(())
}
