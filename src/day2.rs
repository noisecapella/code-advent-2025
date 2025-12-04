use wasm_bindgen::prelude::*;
use std::iter::Map;
use std::str::Split;
use web_sys::console::*;

fn parse_ranges(input: &str) -> Result<Vec<(u128, u128)>, JsError> {
    let pieces = input.split(",");
    pieces.map(|piece| {
        let trimmed_piece = piece.trim();
        let mut numstrs = trimmed_piece.split("-");
        let first: &str = numstrs.next().ok_or_else(|| JsError::new("No first number"))?;
        let second: &str = numstrs.next().ok_or_else(|| JsError::new("No second number"))?;
        let pair: (u128, u128) = (
            str::parse::<u128>(first)?,
            str::parse::<u128>(second)?,
        );
        Ok(pair)
    }).collect()
}

fn calc_num_digits(num: u128) -> u32 {
    let mut num_digits = 0;
    let mut _num = num;
    while _num > 0 {
        _num /= 10;
        num_digits += 1;
    }
    num_digits
}

fn is_invalid_id_part1(num: u128) -> Result<bool, JsError> {
    // check if num is made of two repeating segments

    let num_digits = calc_num_digits(num);
    if num_digits % 2 != 0 {
        Ok(false)
    } else {
        let tens: u128 = 10_u128.checked_pow(num_digits / 2).ok_or_else(|| JsError::new("Overflow"))?;
        let slice = num % tens;
        let copy = num / tens;
        Ok(slice == copy)
    }
}

pub fn day2_part1(input: &str) -> Result<String, JsError> {
    let ranges = parse_ranges(input)?;
    let mut invalid_ids: u128 = 0;

    for range in ranges {
        for num in range.0..=range.1 {
            if is_invalid_id_part1(num)? {
                invalid_ids = invalid_ids.checked_add(num).ok_or_else(
                    || JsError::new("Overflow")
                )?;
            }
        }    
    }
    
    Ok(invalid_ids.to_string())
    
}

fn check_invalid_part2(num: u128, num_digits: u32, num_digit_per_group: u32) -> Result<bool, JsError> {
    let mut copy = num;
    let times = num_digits / num_digit_per_group;
    let tens: u128 = 10_u128.checked_pow(num_digit_per_group).ok_or_else(|| JsError::new("Overflow"))?;
    let slice = num % tens;

    for i in 0..times {
        let _slice = copy % tens;
        if _slice != slice {
            return Ok(false);
            
        } else {
            copy = copy / tens;
        }
    }
    Ok(true)
}

fn is_invalid_id_part2(num: u128) -> Result<bool, JsError> {
    // check if num is made of two repeating segments

    let num_digits = calc_num_digits(num);
    for num_digit_per_group in 1..num_digits {
        
        if num_digits % num_digit_per_group == 0 {
            if check_invalid_part2(num, num_digits, num_digit_per_group)? {
                return Ok(true);
                
            }
        }
    }
    Ok(false)
}

pub fn day2_part2(input: &str) -> Result<String, JsError> {
    let ranges = parse_ranges(input)?;
    let mut invalid_ids: u128 = 0;

    for range in ranges {
        for num in range.0..=range.1 {
            if is_invalid_id_part2(num)? {
                invalid_ids = invalid_ids.checked_add(num).ok_or_else(
                    || JsError::new("Overflow")
                )?;
            }
        }
    }

    Ok(invalid_ids.to_string())

}