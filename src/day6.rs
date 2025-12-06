use leptos::wasm_bindgen::JsError;

struct Math {
    math: Vec<(Vec<u128>, char)>,
}

fn parse_math_part1(input: &str) -> Result<Math, JsError> {
    let line_count = input.lines().count();

    if line_count == 0 {
        return Err(JsError::new("No lines found"));
    }

    let numbers_result: Result<Vec<Vec<u128>>, JsError> = input.lines().take(line_count - 1).map(|line| {
        let line_numbers: Result<Vec<u128>, JsError> = line.split_whitespace().map(|piece| {
            match str::parse::<u128>(piece) {
                Ok(_parsed) => Ok(_parsed),
                Err(_err) => Err(_err.into())
            }
        }).collect();
        line_numbers
    }).collect();

    let final_line  = input.lines().skip(line_count - 1).next().ok_or_else(|| JsError::new("No final line found"))?;
    let operators_result: Result<Vec<char>, JsError> = final_line.split_whitespace().map(
        |piece| {
            if piece.len() > 1 {
                Err(JsError::new("Operators should be only one character long"))
            } else {
                piece.chars().next().ok_or_else(
                    || JsError::new("No operator found")
                )
            }
        }
    ).collect();

    let numbers = numbers_result?;
    let operators = operators_result?;

    let column_count = operators.len();
    for row in numbers.iter() {
        if row.len() != column_count {
            return Err(JsError::new("All rows should have the same number of columns"));
        }
    }

    let mut transposed_numbers: Vec<Vec<u128>> = operators.iter().map(|_| Vec::new() ).collect();
    for (row_idx, row) in numbers.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            let new_col = transposed_numbers.get_mut(col_idx).ok_or_else(
                || JsError::new("Could not get column")
            )?;
            new_col.push(*col);
        }
    }

    let math: Vec<(Vec<u128>, char)> = transposed_numbers.into_iter().zip(operators.into_iter()).collect();

    Ok(Math {
        math
    })
}

fn parse_math_part2(input: &str) -> Result<Math, JsError> {
    let line_count = input.lines().count();
    let number_lines: Vec<Vec<char>> = input.lines().take(line_count - 1).map(
        |line| line.chars().collect()
    ).collect();

    if line_count == 0 {
        return Err(JsError::new("No lines found"));
    }

    let final_line  = input.lines().skip(line_count - 1).next().ok_or_else(|| JsError::new("No final line found"))?;
    let operators: Vec<(usize, char)> = final_line.chars().enumerate().filter_map(
        |(idx, piece)| {
            if piece != ' ' {
                Some((idx, piece))
            } else {
                None
            }
        }
    ).collect();

    let operator_pairs_result: Result<Vec<((usize, char), usize)>, JsError> = operators.windows(2).map(|pair| {
        let first = pair.get(0).and_then(
            |item| Some(*item)
        );
        let second = pair.get(1).and_then(
            |item| Some(item.0 - 1)
        );
        match first {
            Some(_first) => {
                match second {
                    Some(_second) => {
                        Ok((_first, _second))
                    },
                    None => {
                        Err(JsError::new("unable to find second item"))
                    }
                }
            },
            None => {
                Err(JsError::new("unable to find first item"))
            }
        }
    }).chain([
        // add on last item which is (last, idx at end of line)
        match operators.last() {
            Some(_last) => {
                Ok((*_last, final_line.len()))
            },
            None => Err(JsError::new("unable to find last")),
        }
    ]).collect();
    let operator_pairs = operator_pairs_result?;

    let numbers_result: Result<Vec<(Vec<u128>, char)>, JsError> = operator_pairs.iter().map(|pair| {
        let mut num_vec: Vec<u128> = vec![0; pair.1 - pair.0.0];

        for line in number_lines.iter() {
            for idx in 0..(pair.1 - pair.0.0) {
                let col_idx = idx + pair.0.0;

                let c = line.get(col_idx).ok_or_else(
                    || JsError::new("No char found")
                )?;
                match c {
                    ' ' => {
                        // skip
                    },
                    _ => {
                        match char::to_digit(*c, 10) {
                            Some(_digit) => {
                                let item = num_vec.get_mut(idx).ok_or_else(
                                    || JsError::new("can't access item")
                                )?;

                                *item = (*item * 10) + _digit as u128;
                            },
                            None => {
                                Err(
                                    JsError::new("Unknown char")
                                )?
                            }
                        }
                    }
                };
            }
        }
        Ok((num_vec, pair.0.1))
    }).collect();

    let numbers = numbers_result?;

    Ok(Math {
        math: numbers
    })
}

fn calc_math(math: &Math) -> Result<u128, JsError> {
    math.math.iter().map(|(_numbers, op)| {
        match op {
            '*' => _numbers.iter().try_fold(1u128, |acc, item| {
                acc.checked_mul(*item).ok_or_else(
                    || JsError::new("Overflow in mul")
                )
            }),
            '+' => _numbers.iter().try_fold(0u128, |acc, item| {
                acc.checked_add(*item).ok_or_else(
                    || JsError::new("Overflow in sum")
                )
            }),
            _ => Err(JsError::new("Unknown operator"))
        }
    }).try_fold(0u128, |acc, item| {
        item.and_then(|_item| _item.checked_add(acc).ok_or_else(
            || JsError::new("Overflow in sum total")
        ))
    })
}

pub fn day6_part1(input: &str) -> Result<String, JsError> {
    let math = parse_math_part1(input)?;
    let total = calc_math(&math)?;
    Ok(total.to_string())
}

pub fn day6_part2(input: &str) -> Result<String, JsError> {
    let math = parse_math_part2(input)?;
    let total = calc_math(&math)?;
    Ok(total.to_string())
}

