use leptos::ev::{Event, MouseEvent};
use leptos::prelude::*;
use leptos::wasm_bindgen::JsError;
use leptos::html::HtmlElement;
use leptos::wasm_bindgen::JsCast;
use leptos::logging::error;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

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

fn Main() -> impl IntoView {
    let (input_text, set_input_text) = signal("".to_string());
    let (message, set_message) = signal("".to_string());

    let process = Action::new(move |input: &(u64, u64)| {
        let (day, part) = *input;
        async move {
            let daypart = DAY_PARTS.iter().find(
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
        <div style="display: flex; gap: 10px;">
            <b>Day 1</b>
            <button disabled={disabled} on:click=move |e| { process.dispatch((1, 1)); }>Part 1</button>
            <button disabled={disabled} on:click=move |e| { process.dispatch((1, 2)); }>Part 2</button>
        </div>
        <div style="display: flex; gap: 10px;">
            <b>Day 2</b>
            <button disabled={disabled} on:click=move |e| { process.dispatch((2, 1)); }>Part 1</button>
            <button disabled={disabled} on:click=move |e| { process.dispatch((2, 2)); }>Part 2</button>
        </div>
        <div style="display: flex; gap: 10px;">
            <b>Day 3</b>
            <button disabled={disabled} on:click=move |e| { process.dispatch((3, 1)); }>Part 1</button>
            <button disabled={disabled} on:click=move |e| { process.dispatch((3, 2)); }>Part 2</button>
        </div>
        <div style="display: flex; gap: 10px;">
            <b>Day 4</b>
            <button disabled={disabled} on:click=move |e| { process.dispatch((4, 1)); }>Part 1</button>
            <button disabled={disabled} on:click=move |e| { process.dispatch((4, 2)); }>Part 2</button>
        </div>
        <div style="display: flex; gap: 10px;">
            <b>Day 5</b>
            <button disabled={disabled} on:click=move |e| { process.dispatch((5, 1)); }>Part 1</button>
            <button disabled={disabled} on:click=move |e| { process.dispatch((5, 2)); }>Part 2</button>
        </div>
        <h4>Messages</h4>
        <div>
            <pre>{message}</pre>
        </div>
    }
}

fn main() {

    leptos::mount::mount_to_body(Main)
}
