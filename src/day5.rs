use wasm_bindgen::prelude::*;
use web_sys::console::*;

fn parse_ranges(lines: &mut std::str::Lines) -> Result<Vec<std::ops::Range<u64>>, JsError> {
    lines.take_while(|line| {
        !line.trim().is_empty()
    }).map(|line| {
        let mut split = line.trim().split("-");
        let start: u64 = split.next().ok_or_else(|| {
            JsError::new("Could not read start")
        }).and_then(|s| {
            Ok(str::parse::<u64>(s)?)
        })?;
        let end: u64 = split.next().ok_or_else(|| {
            JsError::new("Could not read start")
        }).and_then(|s| {
            Ok(str::parse::<u64>(s)?)
        })?;
        
        Ok(std::ops::Range {
            start,
            end: end + 1,
        })
    }).collect()
}

fn parse_ingredients(lines: &mut std::str::Lines) -> Result<Vec<u64>, JsError> {
    lines.map(|line| {
        match str::parse::<u64>(line) {
            Ok(_line) => Ok(_line),
            Err(_err) => Err(_err.into())
        }
    }).collect()
}

pub fn day5_part1(input: &str) -> Result<String, JsError> {
    let mut lines = input.lines();
    let ranges = parse_ranges(&mut lines)?;
    let ingredients = parse_ingredients(&mut lines)?;
    
    let count = ingredients.iter().filter_map(|ingredient| {
        ranges.iter().find(|range| {
            range.contains(ingredient)
        })
    }).count();
    
    Ok(count.to_string())
}

fn deoverlap(range: &Vec<std::ops::Range<u64>>) -> Result<Vec<std::ops::Range<u64>>, JsError> {
    let mut clone: Vec<std::ops::Range<u64>> = range.clone();
    clone.sort_by_key(|range| { range.start });
    
    let mut prev: Option<std::ops::Range<u64>> = None;
    let copy: Vec<std::ops::Range<u64>> = clone.iter().filter_map(|item| {
        match prev.clone() {
            Some(_prev) => {
                if item.end > _prev.end {
                    prev = Some(item.clone());
                    Some(std::ops::Range {
                        start: std::cmp::max(item.start, _prev.end),
                        end: item.end
                    })
                } else {
                    None
                }
            },
            None => {
                prev = Some(item.clone());
                Some(item.clone())
            }
        }
    }).collect();
    
    Ok(copy)
}

pub fn day5_part2(input: &str) -> Result<String, JsError> {
    let mut lines = input.lines();
    let ranges = parse_ranges(&mut lines)?;

    let nonoverlapping = deoverlap(&ranges)?;
    
    let sum: u64 = nonoverlapping.iter().map(|range| {
        range.end - range.start
    }).sum();
    
    Ok(sum.to_string())
}

