use leptos::{view, IntoView};
use leptos::leptos_dom::error;
use leptos::prelude::{signal, Action};
use leptos::wasm_bindgen::JsError;
use leptos::prelude::*;

pub mod days;

pub fn Main() -> impl IntoView {
    let (input_text, set_input_text) = signal("".to_string());
    let (message, set_message) = signal("".to_string());

    let process = Action::new(move |input: &(u64, u64)| {
        let (day, part) = *input;
        async move {
            let daypart = crate::days::DAY_PARTS.iter().find(
                |dp| dp.day == day && dp.part == part
            ).ok_or_else(||
                JsError::new(format!("No function found for day {} part {}", day, part).as_str())
            );
            let result = daypart.and_then(|_daypart| {
                (_daypart.func)(input_text.get().as_ref())
            });
            match result {
                Ok(_result) => {
                    set_message.set(_result);
                },
                Err(_err) => {
                    set_message.set("Error, see console".to_string());
                    error!("Error: {:?}", _err);
                }
            };
        }
    });

    let disabled  = move || {
        let _disabled: bool = process.pending().get();
        if _disabled {
            Some("true")
        } else {
            None
        }
    };

    view! {
        <div>
            <h4 style="margin-bottom: 20px;">Input:</h4>
            <textarea
                rows={20}
                prop:value={input_text}
                style="margin-bottom: 20px;"
                on:input:target={
                    move |e| {
                        set_input_text.set(e.target().value());
                    }
                }
            />
        </div>
        {move || {
            days::DAY_PARTS.into_iter().filter(|day| day.part == 1).map(|daypart| {
                let day = daypart.day;

                view! {
                    <div style="display: flex; gap: 10px;">
                        <b>"Day "{day}</b>
                        <button disabled={disabled} on:click=move |e| { process.dispatch((day, 1)); }>Part 1</button>
                        <button disabled={disabled} on:click=move |e| { process.dispatch((day, 2)); }>Part 2</button>
                    </div>
                }
            }).collect::<Vec<_>>()
        }}
        <h4>Messages</h4>
        <div>
            <pre>{message}</pre>
        </div>
    }
}
