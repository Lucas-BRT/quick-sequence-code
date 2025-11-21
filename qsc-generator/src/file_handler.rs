use crate::constants::DEFAULT_QRCODE_MAX_DATA_CAPACITY;
use crate::dom::show_error_state;
use crate::dom::{clear_container, create_canvas, show_loading_state};
use crate::utils::get_document;
use js_sys::{ArrayBuffer, Uint8Array};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Event, File, FileReader, console, window};

pub fn log_file_info(file_name: &str, file_size: usize) {
    console::log_1(&format!("Processing file: {} ({} bytes)", file_name, file_size).into());
}

pub fn create_canvas_sequence(data: &[u8]) -> Result<(), JsValue> {
    let file_size = data.len();
    let canvas_count = file_size.div_ceil(DEFAULT_QRCODE_MAX_DATA_CAPACITY);
    console::log_1(&format!("canvas_count: {}", canvas_count).into());

    clear_container("canvas-container")?;

    create_canvas_async(0, canvas_count)?;

    Ok(())
}

fn create_canvas_async(index: usize, total: usize) -> Result<(), JsValue> {
    if index >= total {
        console::log_1(&"All canvas elements created".into());
        return Ok(());
    }

    let document = get_document()?;
    let container = document
        .get_element_by_id("canvas-container")
        .ok_or("Canvas container not found")?;

    let canvas = create_canvas(&format!("canvas-{}", index), None, None, None)?;
    container.append_child(&canvas)?;

    let closure = Closure::wrap(Box::new(move || {
        let _ = create_canvas_async(index + 1, total);
    }) as Box<dyn FnMut()>);

    let window = window().ok_or("No global window exists")?;
    window.set_timeout_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        10,
    )?;
    closure.forget();

    Ok(())
}

pub fn process_selected_file(file_name: &str, data: &[u8]) -> Result<(), JsValue> {
    log_file_info(file_name, data.len());

    console::log_1(&data.len().to_string().into());

    show_loading_state()?;
    create_canvas_sequence(data)?;

    console::log_1(&"QR codes generated successfully".into());
    Ok(())
}

pub fn handle_file_change(file: &File) -> Result<(), JsValue> {
    let file_reader = FileReader::new()?;
    let file_name = file.name();

    let closure = Closure::wrap(Box::new(move |event: Event| {
        if let Some(target) = event.target() {
            let file_reader = target
                .dyn_into::<FileReader>()
                .map_err(|reason| {
                    console::error_1(
                        &format!("Failed to instantiate file reader: {:?}", reason).into(),
                    )
                })
                .expect("failed to instantiate file reader");

            let result = file_reader
                .result()
                .map_err(|err| {
                    console::error_1(&format!("Failed to read file: {:?}", err).into());
                })
                .expect("failed to read file");

            let array_buffer = result
                .dyn_into::<ArrayBuffer>()
                .expect("failed to parse result into ArrayBuffer");

            let uint8_array = Uint8Array::new(&array_buffer);
            let data = uint8_array.to_vec();

            if let Err(e) = process_selected_file(&file_name, &data) {
                console::error_1(&format!("Error processing file: {:?}", e).into());
                show_error_state(None).expect("failed to show error state");
            } else {
                console::log_1(&"File processed successfully".into());
            }
        }
    }) as Box<dyn FnMut(Event)>);

    file_reader.set_onload(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    file_reader.read_as_array_buffer(file)?;

    Ok(())
}
