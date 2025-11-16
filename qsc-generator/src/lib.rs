use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn process_file(file: &[u8]) -> usize {
    file.iter().fold(0usize, |acc, &byte| acc + byte as usize)
}

#[wasm_bindgen]
pub fn hello() -> String {
    "hello, world!".to_string()
}
