mod day1;

use wasm_bindgen::prelude::*;
use web_sys::{HtmlButtonElement, HtmlInputElement};

fn add_day(list: &web_sys::Element, day: u64, process: fn(&str) -> String) -> Result<(), JsValue> {
    let document = get_document()?;
    let li = document.create_element("li")?;
    let label_str = format!("Day {day}");
    let label = document.create_element("label")?;
    label.set_text_content(Some(&label_str));
    li.append_child(&label)?;

    let text_input_id = format!("day-{day}-input");
    let result_span_id = format!("day-{day}-result");
    let result_span_id_for_closure = result_span_id.to_string();

    let text_input = document.create_element("input")?;
    text_input.set_attribute("id", &text_input_id)?;
    li.append_child(&text_input)?;

    let button_element = document.create_element("button")?;
    let button = button_element.dyn_ref::<HtmlButtonElement>().ok_or_else(
        || JsError::new("Could not cast to button")
    )?;
    let on_click = Closure::<dyn FnMut(web_sys::Event) -> Result<(), JsError>>::new(
        move |e: web_sys::Event| -> Result<(), JsError> {
            web_sys::console::log_2(&"Clicked".into(), &text_input_id.to_string().into());
            e.prevent_default();

            // need to get document from web_sys since closure has a static lifetime
            let _document = get_document()?;

            let input_element = _document.get_element_by_id(&text_input_id).ok_or_else(
                || JsError::new("Could not find text element")
            )?;
            let input = input_element.dyn_ref::<HtmlInputElement>().ok_or_else(
                || JsError::new("Id was not for a text element")
            )?;
            
            let result = process(&input.value());
            let text_element = _document.get_element_by_id(&result_span_id_for_closure).ok_or_else(
                || JsError::new("Could not find span")
            )?;
            text_element.set_inner_html(&result);

            Ok(())
        }
    );
    button.set_onclick(Some(on_click.as_ref().unchecked_ref()));
    button.set_inner_html("Click");
    li.append_child(&button)?;
    on_click.forget();

    let result_span = document.create_element("span")?;
    result_span.set_attribute("id", &result_span_id)?;
    li.append_child(&result_span)?;

    list.append_child(&li)?;

    Ok(())
}

fn get_window() -> Result<web_sys::Window, JsError> {
    web_sys::window().ok_or_else(|| JsError::new("no global `window` exists"))
}

fn get_document() -> Result<web_sys::Document, JsError> {
    get_window()?.document().ok_or_else(|| JsError::new("should have a document on window"))
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let document = get_document()?;
    let body = document.body().ok_or_else(|| JsError::new("document should have a body"))?;

    let description = document.create_element("h3")?;
    description.set_text_content(Some("Advent of Code 2025"));
    body.append_child(&description)?;

    let list = document.create_element("ul")?;

    add_day(&list, 1, day1::day1)?;

    body.append_child(&list)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

}
