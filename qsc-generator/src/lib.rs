use qrcode::{Color, QrCode};
use wasm_bindgen::prelude::*;
use web_sys::{console, js_sys::Boolean};

pub fn parse_into_qrcode(stream: &[u8]) -> Result<Vec<Boolean>, String> {
    let qrcode = match QrCode::new(stream) {
        Ok(code) => code,
        Err(e) => return Err(format!("Failed to create QR code: {}", e)),
    };

    let a = qrcode
        .into_colors()
        .into_iter()
        .map(|color| color == Color::Light)
        .map(|b| b.into())
        .collect::<Vec<Boolean>>();

    Ok(a)
}

#[wasm_bindgen]
pub fn process_file(file: &[u8]) -> Vec<Boolean> {
    file.iter().fold(0usize, |acc, &byte| acc + byte as usize);
    match parse_into_qrcode(file) {
        Ok(colors) => colors,
        Err(e) => {
            console::error_1(&e.into());
            vec![]
        }
    }
}

#[wasm_bindgen]
pub fn hello() -> String {
    "hello, world!".to_string()
}
