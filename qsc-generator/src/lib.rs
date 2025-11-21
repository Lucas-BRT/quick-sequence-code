use wasm_bindgen::prelude::*;

mod app;
mod constants;
mod dom;
mod events;
mod file_handler;
mod qrcode;
mod utils;

pub use app::{init_app, update_initialization_message};
pub use constants::*;
pub use dom::{
    add_qrcode_elements_to_dom, append_elements_to_container, append_elements_with_inner_html,
    clear_container, create_canvas, create_error_message, create_loading_message,
    create_qrcode_elements, create_qrcode_elements_as_html, create_status_message,
    show_error_state, show_loading_state,
};
pub use events::setup_file_selector;
pub use file_handler::{handle_file_change, log_file_info, process_selected_file};
pub use qrcode::render_qrcode_canvas;

#[wasm_bindgen(start)]
pub fn main() {
    web_sys::console::log_1(&"WASM module loaded - Quick Sequence Code Generator".into());

    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
