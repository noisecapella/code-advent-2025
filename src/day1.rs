use wasm_bindgen::prelude::*;

pub fn day1(input: &str) -> Result<String, JsError> {
    let mut start: i64 = 50;
    let mut zeros: i64 = 0;
    for line in input.lines() {

        let numstr: String = line.chars().skip(1).collect();
        let num = str::parse::<i64>(numstr.as_str())?;
        if line.starts_with("L") {
            start -= num;
        } else {
            start += num;
        }
        
        while start < 0 {
            start += 100;
        }
        while start >= 100 {
            start -= 100;
        }
        if start == 0 {
            zeros += 1;
        }
    }
    Ok(format!("end: {start}, zeros: {zeros}"))
}