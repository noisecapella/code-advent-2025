use leptos::{view, IntoView};
use leptos::leptos_dom::error;
use leptos::prelude::{signal, Action};
use leptos::wasm_bindgen::JsError;
use leptos::prelude::*;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

pub struct DayPart {
    pub day: u64,
    pub part: u64,
    pub func: fn(&str) -> Result<String, JsError>,
}

pub const DAY_PARTS: [DayPart; 14] = [
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
    DayPart { day: 6, part: 1, func: day6::day6_part1 },
    DayPart { day: 6, part: 2, func: day6::day6_part2 },
    DayPart { day: 7, part: 1, func: day7::day7_part1 },
    DayPart { day: 7, part: 2, func: day7::day7_part2 },
];

