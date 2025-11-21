use crate::dom::show_error_state;
use crate::dom::{clear_container, create_canvas, show_loading_state};
use crate::qrcode::{get_max_qr_capacity, render_qrcode_canvas};
use crate::utils::get_document;
use js_sys::{ArrayBuffer, Uint8Array};
use std::sync::Arc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Event, File, FileReader, console, window};

pub fn log_file_info(file_name: &str, file_size: usize) {
    console::log_1(&format!("Processing file: {} ({} bytes)", file_name, file_size).into());
}

pub fn create_canvas_sequence(data: &[u8]) -> Result<(), JsValue> {
    let file_size = data.len();
    let max_capacity = get_max_qr_capacity();
    console::log_1(&format!("Detected max QR capacity: {} bytes", max_capacity).into());

    let canvas_count = file_size.div_ceil(max_capacity);
    console::log_1(&format!("canvas_count: {}", canvas_count).into());

    clear_container("canvas-container")?;

    let data_arc = Arc::new(data.to_vec());

    create_canvas_async(0, canvas_count, data_arc, max_capacity)?;

    Ok(())
}

fn create_canvas_async(
    index: usize,
    total: usize,
    data: Arc<Vec<u8>>,
    max_capacity: usize,
) -> Result<(), JsValue> {
    if index >= total {
        console::log_1(&"All canvas elements created and rendered".into());
        return Ok(());
    }

    let document = get_document()?;
    let container = document
        .get_element_by_id("canvas-container")
        .ok_or("Canvas container not found")?;

    let canvas_id = format!("canvas-{}", index);
    console::log_1(&format!("Creating canvas with ID: {}", canvas_id).into());

    let canvas = create_canvas(&canvas_id, None, None, None)?;
    console::log_1(&format!("Canvas created successfully: {}", canvas_id).into());

    container.append_child(&canvas)?;
    console::log_1(&format!("Canvas appended to DOM: {}", canvas_id).into());

    let chunk_start = index * max_capacity;
    let chunk_end = ((index + 1) * max_capacity).min(data.len());
    let mut chunk_data = &data[chunk_start..chunk_end];

    if chunk_data.len() > max_capacity {
        chunk_data = &chunk_data[..max_capacity];
    }

    console::log_1(
        &format!(
            "Canvas {}: chunk_start={}, chunk_end={}, chunk_size={}, max_capacity={}",
            index,
            chunk_start,
            chunk_end,
            chunk_data.len(),
            max_capacity
        )
        .into(),
    );

    match render_qrcode_canvas(&canvas_id, chunk_data) {
        Ok(_) => {
            console::log_1(&format!("Canvas {} rendered successfully", index).into());
        }
        Err(e) => {
            console::error_1(&format!("Failed to render canvas {}: {:?}", index, e).into());
        }
    }

    console::log_1(&format!("Setting timeout for next canvas {}", index + 1).into());

    let data_for_closure = Arc::clone(&data);
    let closure = Closure::wrap(Box::new(move || {
        console::log_1(&format!("Timeout triggered for canvas {}", index + 1).into());
        match create_canvas_async(
            index + 1,
            total,
            Arc::clone(&data_for_closure),
            max_capacity,
        ) {
            Ok(_) => {
                console::log_1(&format!("Successfully processed canvas {}", index + 1).into());
            }
            Err(e) => {
                console::error_1(&format!("Error in canvas {}: {:?}", index + 1, e).into());
            }
        }
    }) as Box<dyn FnMut()>);

    let window = window().ok_or("No global window exists")?;
    match window
        .set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 10)
    {
        Ok(_) => {
            console::log_1(&format!("Timeout set successfully for canvas {}", index + 1).into());
        }
        Err(e) => {
            console::error_1(&format!("Failed to set timeout: {:?}", e).into());
            return Err(e);
        }
    }
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
