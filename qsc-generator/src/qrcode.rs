use crate::DEFAULT_QR_CODE_VERSION;
use qrcode::{Color, QrCode};
use qrcode::{EcLevel, Version};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, console, window};

pub fn get_max_qr_capacity() -> usize {
    let mut low = 1;
    let mut high = 3000;
    let mut max_capacity = 2953;

    while low <= high {
        let mid = (low + high) / 2;

        let mut test_data = Vec::with_capacity(mid);
        let csv_pattern = b"item,value,timestamp,category,description\n";
        let row_pattern = b"data123,456.78,2024-01-01,category1,some description text\n";

        while test_data.len() < mid {
            if test_data.is_empty() {
                test_data.extend_from_slice(
                    &csv_pattern[..csv_pattern.len().min(mid - test_data.len())],
                );
            } else {
                test_data.extend_from_slice(
                    &row_pattern[..row_pattern.len().min(mid - test_data.len())],
                );
            }
        }
        test_data.truncate(mid);

        if QrCode::with_version(
            &test_data,
            Version::Normal(DEFAULT_QR_CODE_VERSION),
            EcLevel::L,
        )
        .is_ok()
        {
            max_capacity = mid;
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    let safety_margin = (max_capacity as f64 * 0.2) as usize;
    let safe_capacity = max_capacity.saturating_sub(safety_margin);

    console::log_1(&format!("Raw max QR capacity: {} bytes", max_capacity).into());
    console::log_1(&format!("Safe QR capacity (20% margin): {} bytes", safe_capacity).into());
    safe_capacity
}

pub fn render_qrcode_canvas(canvas_id: &str, data: &[u8]) -> Result<(), JsValue> {
    console::log_1(&format!("Starting render for canvas: {}", canvas_id).into());

    let window = window().ok_or("No global `window` exists")?;
    let document = window
        .document()
        .ok_or("Should have a document on window")?;

    console::log_1(&format!("Looking for canvas element: {}", canvas_id).into());
    let canvas = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| {
            console::error_1(&format!("Canvas with id '{}' not found in DOM", canvas_id).into());
            format!("Canvas with id '{}' not found", canvas_id)
        })?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| {
            console::error_1(&format!("Element '{}' is not a canvas", canvas_id).into());
            "Element is not a canvas"
        })?;

    console::log_1(&format!("Canvas found, getting 2D context for: {}", canvas_id).into());
    let context = canvas
        .get_context("2d")
        .map_err(|_| "Failed to get 2d context")?
        .ok_or("No 2d context")?
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(|_| "Failed to cast to CanvasRenderingContext2d")?;

    console::log_1(
        &format!(
            "Attempting to create QR code with {} bytes for canvas: {}",
            data.len(),
            canvas_id
        )
        .into(),
    );

    let qrcode =
        match QrCode::with_version(data, Version::Normal(DEFAULT_QR_CODE_VERSION), EcLevel::L) {
            Ok(code) => {
                console::log_1(
                    &format!(
                        "QR code created successfully for {} bytes on canvas: {}",
                        data.len(),
                        canvas_id
                    )
                    .into(),
                );
                code
            }
            Err(e) => {
                console::error_1(
                    &format!(
                        "Failed to create QR code with {} bytes for canvas {}: {}",
                        data.len(),
                        canvas_id,
                        e
                    )
                    .into(),
                );
                return Err(JsValue::from_str(&format!(
                    "Failed to create QR code for canvas {}: {}",
                    canvas_id, e
                )));
            }
        };

    let size = qrcode.width();
    console::log_1(&format!("QR code size: {}x{} for canvas: {}", size, size, canvas_id).into());

    let modules = qrcode
        .into_colors()
        .into_iter()
        .map(|color| color == Color::Light)
        .collect::<Vec<bool>>();

    let canvas_size = canvas.width();
    let module_size = canvas_size as f64 / size as f64;

    console::log_1(
        &format!(
            "Rendering QR code on canvas {} ({}x{} pixels, module_size: {})",
            canvas_id, canvas_size, canvas_size, module_size
        )
        .into(),
    );

    context.clear_rect(0.0, 0.0, canvas_size as f64, canvas_size as f64);

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

    console::log_1(&format!("QR code rendered successfully on canvas: {}", canvas_id).into());
    Ok(())
}
