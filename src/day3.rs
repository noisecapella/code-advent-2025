use leptos::wasm_bindgen::JsError;

fn sort_chars(line: &str) -> Result<Vec<(usize, u32)>, JsError> {
    let mut option_vec: Option<Vec<(usize, u32)>> = line.chars().enumerate().map(|(i, c)| {
        let digit = char::to_digit(c, 10);
        match digit {
            Some(digit) => Some((i, digit as u32)),
            None => None
        }
    }).collect();

    let mut vec: Vec<(usize, u32)> = option_vec.ok_or_else(|| JsError::new("Some characters did not parse correctly"))?;

    vec.sort_by_key(|(i, c)| (*c, -(*i as i32)));
    vec.reverse();
    Ok(vec)
}

fn calc_joltage(line: &str, sum: u128, pick: (usize, u32), depth: usize, max_depth: usize, sorted: &Vec<(usize, u32)>) -> Result<Option<u128>, JsError> {
    let new_sum = sum.checked_mul(10).and_then(|x| x.checked_add(pick.1 as u128)).ok_or_else(
        || JsError::new("Overflow")
    )?;
    if depth + 1 == max_depth {
        return Ok(Some(new_sum));
    }

    for item in sorted {
        if item.0 > pick.0 {
            let find = calc_joltage(line, new_sum, *item, depth + 1, max_depth, sorted)?;
            match find {
                Some(_find) => return Ok(find),
                None => {}
            }
        }
    }

    Ok(None)
}

fn calc_joltage_initial(line: &str, max_depth: usize) -> Result<u128, JsError> {
    let sorted = sort_chars(line)?;

    let result: Option<Result<u128, JsError>> = sorted.iter().find_map(|item| {

        let joltage = calc_joltage(line, 0, *item, 0, max_depth, &sorted);
        match joltage {
            Ok(Some(joltage)) => Some(Ok(joltage)),
            Ok(None) => None,
            Err(err) => Some(Err(err))
        }
    });

    result.ok_or_else(|| JsError::new("No result found for line"))?
    
}

pub fn day3_part1(input: &str) -> Result<String, JsError> {
    let joltage_sum: u128 = input.lines().filter(|line| {
        line.trim().len() > 0
    }).map(|line| {
        calc_joltage_initial(line, 2)
    }).sum::<Result<u128, JsError>>()?;

    Ok(joltage_sum.to_string())
}

pub fn day3_part2(input: &str) -> Result<String, JsError> {
    let joltage_sum: u128 = input.lines().filter(|line| {
        line.trim().len() > 0
    }).map(|line| {
        calc_joltage_initial(line, 12)
    }).sum::<Result<u128, JsError>>()?;

    Ok(joltage_sum.to_string())
}
