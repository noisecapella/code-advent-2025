mod day1;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use js_sys::JsString;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlTextAreaElement, Worker};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
struct Message {
    day: u64,
    message_type: String,
    message: String
}

fn add_day(worker_handle: Rc<RefCell<Worker>>, list: &web_sys::Element, day: u64) -> Result<(), JsValue> {
    let document = get_document()?;
    let li = document.create_element("li")?;
    let label_str = format!("Day {day}");
    let label = document.create_element("label")?;
    label.set_text_content(Some(&label_str));
    li.append_child(&label)?;

    let br = document.create_element("br")?;
    li.append_child(&br)?;

    let text_input_id = format!("day-{day}-input");
    let button_id = format!("day-{day}-button");
    let button_id_clone = button_id.to_string();
    let result_span_id = format!("day-{day}-result");

    let text_input = document.create_element("textarea")?;
    text_input.set_attribute("id", &text_input_id)?;
    text_input.set_attribute("rows", "20")?;
    li.append_child(&text_input)?;

    let br2 = document.create_element("br")?;
    li.append_child(&br2)?;

    let button_element = document.create_element("button")?;
    let button = button_element.dyn_ref::<HtmlButtonElement>().ok_or_else(
        || JsError::new("Could not cast to button")
    )?;
    let on_click = Closure::<dyn FnMut(web_sys::Event) -> Result<(), JsError>>::wrap(
        Box::new(move |e: web_sys::Event| -> Result<(), JsError> {
            web_sys::console::log_2(&"Clicked".into(), &text_input_id.to_string().into());
            e.prevent_default();

            // need to get document from web_sys since closure has a static lifetime
            let _document = get_document()?;

            let input_element = _document.get_element_by_id(&text_input_id).ok_or_else(
                || JsError::new("Could not find text element")
            )?;
            let input = input_element.dyn_ref::<HtmlTextAreaElement>().ok_or_else(
                || JsError::new("Id was not for a text element")
            )?;

            web_sys::console::log_1(&"Serializing message...".into());
            let message_string: String = input.value().into();
            let message = Message {
                message_type: "request".to_string(),
                day: day,
                message: message_string
            };
            let json: String = serde_json::to_string(&message).or_else(
                |_| Err(JsError::new("Could not serialize message"))
            )?;
            let json_clone = json.to_string();
            web_sys::console::log_2(&"Serialized".into(), &json_clone.into());

            let worker = &*worker_handle.borrow();
            worker.post_message(&json.into()).or_else(
                |_| Err(JsError::new("Could not post message"))
            )?;
            web_sys::console::log_1(&"Sent request".into());

            let _button_element = _document.get_element_by_id(&button_id_clone).ok_or_else(
                || JsError::new("Could not find button element")
            )?;
            let button = _button_element.dyn_ref::<HtmlButtonElement>().ok_or_else(
                || JsError::new("Id was not for a button element")
            )?;
            button.set_attribute("disabled", "1").or_else(
                |_| Err(JsError::new("Could not disable button"))
            )?;

            Ok(())
        }
        )
    );
    button.set_onclick(Some(on_click.as_ref().unchecked_ref()));
    button.set_inner_html("Process Day 1");
    button.set_attribute("id", &button_id.as_str())?;
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

#[wasm_bindgen]
pub fn run(worker: &Worker) -> Result<(), JsValue> {
    let worker = Rc::new(RefCell::new(Worker::new("./worker.js")?));
    let worker_clone = worker.clone();

    let worker_handle = &*worker.borrow();
    let worker_closure = Closure::<dyn FnMut(web_sys::MessageEvent) -> Result<(), JsError>>::new(
        move |message_event: web_sys::MessageEvent| -> Result<(), JsError> {
            web_sys::console::log_1(&"Received response".into());
            let data = message_event.data();
            let js_string = data.dyn_ref::<JsString>().ok_or_else(
                || JsError::new("Could not cast object to string")
            )?;
            let string: String = js_string.into();
            let obj: Message = serde_json::from_str(&string).or_else(
                |_| Err(JsError::new("Could not parse message"))
            )?;
            if obj.message_type != "response" {
                Err(JsError::new("Unexpected response message_type").into())
            } else {
                let _document = get_document()?;
                let day = obj.day;
                let result_span_id = format!("day-{day}-result");
                let result_span = _document.get_element_by_id(&result_span_id).ok_or_else(
                    || JsError::new("Could not find result span")
                )?;
                result_span.set_text_content(Some(&obj.message));


                let day = obj.day;
                let _button_id = format!("day-{day}-button");
                let _button_element = _document.get_element_by_id(&_button_id).ok_or_else(
                    || JsError::new("Could not find button element")
                )?;
                let _button = _button_element.dyn_ref::<HtmlButtonElement>().ok_or_else(
                    || JsError::new("Id was not for a button element")
                )?;
                _button.remove_attribute("disabled").or_else(
                    |_| Err(JsError::new("Could not remove disabled attribute"))
                )?;

                Ok(())
            }
        }
    );
    worker_handle.set_onmessage(Some(
        worker_closure.as_ref().unchecked_ref()
    ));
    worker_closure.forget();

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let document = get_document()?;
    let body = document.body().ok_or_else(|| JsError::new("document should have a body"))?;

    let description = document.create_element("h3")?;
    description.set_text_content(Some("Advent of Code 2025"));
    body.append_child(&description)?;

    let processing_div = document.create_element("div")?;
    processing_div.set_attribute("id", "processing")?;
    body.append_child(&processing_div)?;

    let list = document.create_element("ul")?;

    add_day(worker_clone, &list, 1)?;

    body.append_child(&list)?;

    Ok(())
}


#[wasm_bindgen]
pub fn run_worker(event: &web_sys::Event, post_message: &js_sys::Function) -> Result<(), JsValue> {
    web_sys::console::log_1(&"Worker received event".into());


    let message_event = event.dyn_ref::<web_sys::MessageEvent>().ok_or_else(
        || JsError::new("Could not cast to MessageEvent")
    )?;

    let data = message_event.data();
    let js_string = data.dyn_ref::<JsString>().ok_or_else(
        || JsError::new("Could not cast object to string")
    )?;
    let string: String = js_string.into();
    let obj: Message = serde_json::from_str(&string).or_else(
        |_| Err(JsError::new("Could not parse message"))
    )?;
    if obj.message_type != "request" {
        Err(JsError::new("Unexpected message_type").into())
    } else {
        let result = match obj.day {
            1 => Ok(day1::day1(&obj.message)),
            _ => Err(JsError::new("Unexpected day"))
        }?;
        let message = Message {
            day: obj.day,
            message_type: "response".to_string(),
            message: result,
        };

        let json = serde_json::to_string(&message).or_else(
            |_| Err(JsError::new("Could not serialize result"))
        )?;
        web_sys::console::log_1(&"Worker sent response".into());
        post_message.call1(&JsValue::NULL, &JsValue::from_str(&json))?;

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

}
