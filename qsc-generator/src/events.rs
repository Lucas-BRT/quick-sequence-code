use crate::dom::show_error_state;
use crate::file_handler::handle_file_change;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Event, HtmlInputElement, console};

pub fn setup_file_selector() -> Result<(), JsValue> {
    use crate::utils::get_document;

    let document = get_document()?;
    let file_selector = document
        .get_element_by_id("file-selector")
        .ok_or("File selector element not found")?
        .dyn_into::<HtmlInputElement>()?;

    let closure = Closure::wrap(Box::new(move |event: Event| {
        let target = event.target().expect("failed to get event target");
        let input = target
            .dyn_into::<HtmlInputElement>()
            .expect("failed to parse target into a HtmlInputElement");
        let files = input.files();

        if let Some(files) = files
            && files.length() > 0
        {
            let file = files.get(0);

            if let Some(file) = file {
                if let Err(e) = handle_file_change(&file) {
                    console::error_1(&format!("Error handling file change: {:?}", e).into());
                    show_error_state(None).expect("failed to set error state");
                }
            } else {
                console::error_1(&"File is None".into());
            }
        }
    }) as Box<dyn FnMut(Event)>);

    file_selector.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}
