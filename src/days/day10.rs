use std::collections::HashSet;
use leptos::wasm_bindgen::JsError;
use leptos::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
struct Indicator {
    val: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
struct Mask {
    val: u32,
}

#[derive(Clone, Debug)]
struct Machine {
    indicator_goal: Indicator,
    wiring: Vec<Mask>,
    joltage: Vec<u32>,
}

fn toggle_indicator(indicator: Indicator, mask: Mask) -> Indicator {
    Indicator {
        val: indicator.val ^ mask.val,
    }
}

fn parse_indicator(str: &str) -> Result<Indicator, JsError> {
    let indicator: Vec<bool> = str.chars().filter_map(|c| match c {
        '[' | ']' => {
            None
        },
        '.' => Some(Ok(false)),
        '#' => Some(Ok(true)),
        _ => Some(Err(JsError::new(&format!("Unexpected character {}", c)))),
    }).collect::<Result<Vec<bool>, JsError>>()?;

    let num = indicator.iter().enumerate().fold(0, |acc, (idx, item)| {
        acc | (if *item { 1u32 << idx } else { 0 })
    });
    Ok(Indicator { val: num })
}

fn parse_wiring(str: &[&str]) -> Result<Vec<Mask>, JsError> {
    let wiring: Vec<Mask> = str.iter().map(|piece| {
        let nums = piece.split(",").map(|item| {
            str::parse::<u32>(
                item.replace("(", "").replace(")", "").as_str()
            ).or_else(|_| Err(JsError::new("Couldn't parse wiring")))
        }).collect::<Result<Vec<u32>, JsError>>()?;

        let val = nums.iter().fold(0, |acc, item| {
            acc | (1u32 << item)
        });
        Ok(Mask { val })
    }).collect::<Result<Vec<Mask>, JsError>>()?;

    Ok(wiring)
}

fn parse_joltage(str: &str) -> Result<Vec<u32>, JsError> {
    let joltage: Vec<u32> = str.replace("{", "").replace("}", "").split(",").map(|piece| {
        str::parse::<u32>(piece).or_else(|_| Err(JsError::new("Couldn't parse joltage")))
    }).collect::<Result<Vec<u32>, JsError>>()?;
    Ok(joltage)
}

fn parse_input(input: &str) -> Result<Vec<Machine>, JsError> {
    let machines = input.lines().map(|line| {
        let pieces: Vec<&str> = line.split_whitespace().collect();
        let first = pieces.first().ok_or_else(|| JsError::new("couldn't find first" ))?;
        let middle = pieces.get(1..pieces.len() - 1).ok_or_else(|| JsError::new("couldn't find middle" ))?;
        let last = pieces.last().ok_or_else(|| JsError::new("couldn't find last" ))?;

        Ok(Machine {
            indicator_goal: parse_indicator(first)?,
            wiring: parse_wiring(middle)?,
            joltage: parse_joltage(last)?,
        })
    }).collect::<Result<Vec<Machine>, JsError>>()?;

    Ok(machines)
}

fn calc_min_presses(machine: &Machine) -> u32 {
    let mut current = HashSet::new();
    current.insert(Indicator { val: 0 });
    let mut depth = 0;

    loop {
        let mut next = HashSet::new();
        for current_indicator in current {
            for mask in machine.wiring.iter() {
                let new_val = toggle_indicator(current_indicator, *mask);
                if new_val == machine.indicator_goal {
                    return depth + 1;
                }
                next.insert(new_val);
            }
        }
        depth += 1;
        current = next;
    }
}

pub fn day10_part1(input: &str) -> Result<String, JsError> {
    let machines = parse_input(input)?;
    let sum = machines.iter().map(|machine| {
        calc_min_presses(machine)
    }).sum::<u32>();
    Ok(sum.to_string())
    // Ok(format!("{:?}", machines))
}

pub fn day10_part2(input: &str) -> Result<String, JsError> {
    Ok("".to_string())
}
