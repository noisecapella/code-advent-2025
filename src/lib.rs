mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use js_sys::JsString;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlTextAreaElement, Worker};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
struct Message {
    day: u64,
    part: u64,
    message_type: String,
    message: String
}

fn format_day_button(day: u64, part: u64) -> String {
    format!("day-{day}-{part}-button")
}

fn format_day_result(day: u64, part: u64) -> String {
    format!("day-{day}-{part}-result")
}

struct DayPart {
    day: u64,
    part: u64,
    func: fn(&str) -> Result<String, JsError>,
}

const DAY_PARTS: [DayPart; 10] = [
    DayPart { day: 1, part: 1, func: day1::day1_part1 },
    DayPart { day: 1, part: 2, func: day1::day1_part2 },
    DayPart { day: 2, part: 1, func: day2::day2_part1 },
    DayPart { day: 2, part: 2, func: day2::day2_part2 },
    DayPart { day: 3, part: 1, func: day3::day3_part1 },
    DayPart { day: 3, part: 2, func: day3::day3_part2 },
    DayPart { day: 4, part: 1, func: day4::day4_part1 },
    DayPart { day: 4, part: 2, func: day4::day4_part2 },
    DayPart { day: 5, part: 1, func: day5::day5_part1 },
    DayPart { day: 5, part: 2, func: day5::day5_part2 },
];

fn add_day_part(worker_handle: Rc<RefCell<Worker>>, list: &web_sys::Element, day_part: &DayPart) -> Result<(), JsValue> {
    let document = get_document()?;
    let li = document.create_element("li")?;
    let day = day_part.day;
    let part = day_part.part;
    let label_str = format!("Day {day}, part {part}");
    let label = document.create_element("label")?;
    label.set_text_content(Some(&label_str));
    li.append_child(&label)?;

    let br = document.create_element("br")?;
    li.append_child(&br)?;

    let button_id = format_day_button(day, part);
    let button_id_clone = button_id.to_string();
    let result_span_id = format_day_result(day, part);;

    let br2 = document.create_element("br")?;
    li.append_child(&br2)?;

    let button_element = document.create_element("button")?;
    let button = button_element.dyn_ref::<HtmlButtonElement>().ok_or_else(
        || JsError::new("Could not cast to button")
    )?;
    let on_click = Closure::<dyn FnMut(web_sys::Event) -> Result<(), JsError>>::wrap(
        Box::new(move |e: web_sys::Event| -> Result<(), JsError> {
            e.prevent_default();

            // need to get document from web_sys since closure has a static lifetime
            let _document = get_document()?;

            let input_element = _document.get_element_by_id("day-input").ok_or_else(
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
                part: part,
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

            let result_span_id = format_day_result(day, part);
            let result_span = _document.get_element_by_id(&result_span_id).ok_or_else(
                || JsError::new("Could not find result span")
            )?;
            result_span.set_text_content(Some("Processing..."));

            Ok(())
        }
        )
    );
    button.set_onclick(Some(on_click.as_ref().unchecked_ref()));
    button.set_inner_html(format!("Process Day {day}, part {part}").as_str());
    button.set_attribute("id", &button_id.as_str())?;
    li.append_child(&button)?;
    on_click.forget();

    let br3 = document.create_element("br")?;
    li.append_child(&br3)?;

    let result_span = document.create_element("pre")?;
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
                let part = obj.part;
                let result_span_id = format_day_result(day, part);
                let result_span = _document.get_element_by_id(&result_span_id).ok_or_else(
                    || JsError::new("Could not find result span")
                )?;
                result_span.set_text_content(Some(&obj.message));


                let day = obj.day;
                let _button_id = format_day_button(day, part);
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

    let text_input = document.create_element("textarea")?;
    text_input.set_attribute("id", "day-input")?;
    text_input.set_attribute("rows", "20")?;
    body.append_child(&text_input)?;

    for day_part in DAY_PARTS.iter() {
        add_day_part(worker.clone(), &list, day_part)?;
    }

    body.append_child(&list)?;

    Ok(())
}

fn process_day(day: u64, part: u64, input: &str) -> Result<String, JsError> {
    for _day_part in DAY_PARTS.iter() {
        if day == _day_part.day && part == _day_part.part {
            let func = _day_part.func;
            return Ok(func(input)?);
        }
    }

    Err(JsError::new("Unexpected day or part"))
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
        let result = process_day(
            obj.day, obj.part, &obj.message
        );

        let message_text = result.unwrap_or_else(
            |err| {
                format!("Error: {:?}", JsValue::from(err))
            }
        );
        let message = Message {
            day: obj.day,
            part: obj.part,
            message_type: "response".to_string(),
            message: message_text,
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
