#![feature(portable_simd)]

use leptos::wasm_bindgen::JsError;
use leptos::prelude::*;
use std::simd::prelude::*;

use std::collections::{HashMap, HashSet};
use rustc_hash::FxHashSet;
use rustc_hash::FxHashMap;
use std::ops::{AddAssign, BitAnd, BitOr, BitXor, Mul, MulAssign, SubAssign};
use leptos::prelude::*;
use std::simd::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
struct Indicator {
    val: i16x16,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
struct Mask {
    // each either 1 or 0
    val: i16x16,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
struct Joltage {
    val: i16x16,
}

#[derive(Clone, Debug)]
struct Machine {
    indicator_goal: Indicator,
    wiring: Vec<Mask>,
    joltage: Joltage,
    joltage_len: usize,
}

fn toggle_indicator(indicator: Indicator, mask: Mask) -> Indicator {
    Indicator {
        val: indicator.val.bitxor(mask.val),
    }
}

fn parse_indicator(str: &str) -> Result<Indicator, JsError> {
    let mut indicator: Vec<i16> = str.chars().filter_map(|c| match c {
        '[' | ']' => {
            None
        },
        '.' => Some(Ok(0)),
        '#' => Some(Ok(1)),
        _ => Some(Err(JsError::new(&format!("Unexpected character {}", c)))),
    }).collect::<Result<Vec<i16>, JsError>>()?;
    indicator.resize(16, 0);

    let num = i16x16::from_slice(indicator.as_slice());
    Ok(Indicator { val: num })
}

fn parse_wiring(str: &[&str]) -> Result<Vec<Mask>, JsError> {
    let wiring: Vec<Mask> = str.iter().map(|piece| {
        let nums = piece.split(",").map(|item| {
            str::parse::<i16>(
                item.replace("(", "").replace(")", "").as_str()
            ).or_else(|_| Err(JsError::new("Couldn't parse wiring")))
        }).collect::<Result<Vec<i16>, JsError>>()?;

        let mut mask = Vec::new();
        mask.resize(16, 0);
        for num in nums {
            mask[num as usize] = 1;
        }

        let val = i16x16::from_slice(mask.as_slice());
        Ok(Mask { val })
    }).collect::<Result<Vec<Mask>, JsError>>()?;

    Ok(wiring)
}

fn parse_joltage(str: &str) -> Result<(Joltage, usize), JsError> {
    let mut joltage: Vec<i16> = str.replace("{", "").replace("}", "").split(",").map(|piece| {
        str::parse::<i16>(piece).or_else(
            |err| Err(JsError::new(&format!("Couldn't parse joltage {:?} {:?}", err, piece)))
        )
    }).collect::<Result<Vec<i16>, JsError>>()?;
    let len = joltage.len();
    joltage.resize(16, 0);
    Ok((Joltage {
        val: i16x16::from_slice(joltage.as_slice()),
    }, len))
}

fn parse_input(input: &str) -> Result<Vec<Machine>, JsError> {
    let machines = input.lines().map(|line| {
        let pieces: Vec<&str> = line.split_whitespace().collect();
        let first = pieces.first().ok_or_else(|| JsError::new("couldn't find first" ))?;
        let middle = pieces.get(1..pieces.len() - 1).ok_or_else(|| JsError::new("couldn't find middle" ))?;
        let last = pieces.last().ok_or_else(|| JsError::new("couldn't find last" ))?;

        let joltage_tup = parse_joltage(last)?;
        Ok(Machine {
            indicator_goal: parse_indicator(first)?,
            wiring: parse_wiring(middle)?,
            joltage: joltage_tup.0,
            joltage_len: joltage_tup.1,
        })
    }).collect::<Result<Vec<Machine>, JsError>>()?;

    Ok(machines)
}

fn calc_min_presses_part_1(machine: &Machine) -> u32 {
    let mut current = HashSet::new();
    current.insert(Indicator { val: i16x16::splat(0) });
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
        calc_min_presses_part_1(machine)
    }).sum::<u32>();
    Ok(sum.to_string())
    // Ok(format!("{:?}", machines))
}

fn transpose(masks: &Vec<i64x16>) -> Vec<i64x16> {
    let old_num_rows = masks[0].len();
    let old_num_cols = masks.len();

    let mut transposed = (0..old_num_rows).map(|_| i64x16::splat(0)).collect::<Vec<_>>();
    for row in 0..old_num_rows {
        for col in 0..old_num_cols {
            transposed[row][col] = masks[col][row];
        }
    }

    transposed
}

fn reduce_rows(machine: &Machine) -> Result<Machine, JsError> {
    let mut masks_transposed = transpose(&machine.wiring.iter().map(
        |mask| i64x16::from_array(mask.val.as_array().map(|x| x as i64))
    ).collect());
    let mut joltage = i64x16::from_array(machine.joltage.val.as_array().map(|x| x as i64));
    let mut bound_col = 0;
    let mut bound_row = 0;
    let num_cols = machine.wiring.len();
    let num_rows = machine.joltage_len;
    let mut divisors: Vec<u64> = (0..num_rows).map(|_| 1).collect();

    let print_matrix_transposed = |_masks_transposed: &Vec<i64x16>, _joltage: &i64x16, _divisors: &Vec<u64>| {
        for row in 0..num_rows {
            for col in 0..num_cols {
                print!("{:>5}", _masks_transposed[row][col] /*as f64 / _divisors[row] as f64*/);
            }
            println!(" = {:>5}", _joltage[row] /*as f64 /  _divisors[row] as f64*/);
        }
        println!();
        println!();

    };

    loop {
        // print_matrix_transposed(&masks_transposed, &joltage, &divisors);

        let mut nonzero_col = None;
        for col in bound_col..num_cols {
            for row in bound_row..num_rows {
                if masks_transposed[row][col] != 0 {
                    nonzero_col = Some(col);
                    break;
                }
            }
            if nonzero_col.is_some() {
                break;
            }
        }
        let current_col = match nonzero_col {
            Some(col) => {
                col
            },
            None => {
                break;
            }
        };

        let current_row = bound_row;

        // from https://linearalgebra.math.umanitoba.ca/math1220/section-12.html
        // If the column has nonzero entries, interchange rows, if necessary, to get a nonzero entry on top.
        let mut is_nonzero_col = false;
        for row in current_row..num_rows {
            if masks_transposed[row][current_col] != 0 {
                is_nonzero_col = true;
                break;
            }
        }
        if is_nonzero_col && masks_transposed[current_row][current_col] == 0 {
            for row in (current_row + 1)..num_rows {
                if masks_transposed[row][current_col] != 0 {
                    let old_row = masks_transposed[current_row];
                    let old_joltage = joltage[current_row];
                    let old_divisor = divisors[current_row];
                    let new_row = masks_transposed[row];
                    let new_joltage = joltage[row];
                    let new_divisor = divisors[row];
                    masks_transposed[current_row] = new_row;
                    joltage[current_row] = new_joltage;
                    divisors[current_row] = new_divisor;
                    masks_transposed[row] = old_row;
                    joltage[row] = old_joltage;
                    divisors[row] = old_divisor;
                    break;
                }
            }
        }

        // Change the top entry, if necessary, to make it a 1
        let old_value = masks_transposed[current_row][current_col];
        if old_value < 0 {
            divisors[current_row] *= (old_value * -1) as u64;
            masks_transposed[current_row] *= i64x16::splat(-1);
            joltage[current_row] *= -1i64;
        } else {
            divisors[current_row] *= old_value as u64;
        }
        // masks[current_row] /= old_value;
        // joltage[current_row] /= old_value;



        // For any nonzero entry for other rows, use an elementary row operation to change it to zero.
        for row in 0..num_rows {
            if row == current_row {
                continue;
            }
            if masks_transposed[row][current_col] != 0 {
                // divide row by entry to become 1
                let old_value = masks_transposed[row][current_col];
                if old_value < 0 {
                    masks_transposed[row] *= i64x16::splat(-1);
                    joltage[row] *= -1i64;
                    divisors[row] *= (old_value * -1) as u64;
                } else {
                    divisors[row] *= old_value as u64;
                }

                // make denominators the same before subtraction
                let top_divisor = divisors[current_row];
                let divisor = divisors[row];

                // multiply top row by current row, and vice versa, so that
                // their denominators become equal
                masks_transposed[current_row] *= i64x16::splat(divisor as i64);
                joltage[current_row] *= divisor as i64;
                divisors[current_row] *= divisor;
                masks_transposed[row] *= i64x16::splat(top_divisor as i64);
                joltage[row] *= top_divisor as i64;
                divisors[row] *= top_divisor;

                let _row = masks_transposed[current_row];
                masks_transposed[row] -= _row;
                joltage[row] -= joltage[current_row];

                if old_value < 0 {
                    // we multiplied by -1 to add, do so again
                    masks_transposed[row] *= i64x16::splat(-1);
                    joltage[row] *= -1i64;
                }
            }
        }

        for row in 0..num_rows {
            // reset divisors back to 1
            divisors[row] = 1;

            let gcd = calc_gcd(masks_transposed[row], joltage[row]);
            if gcd != 0 {
                masks_transposed[row] /= i64x16::splat(gcd);
                joltage[row] /= gcd;
            }

        }

        print_matrix_transposed(&masks_transposed, &joltage, &divisors);
        // Now consider the part of the matrix below the top row and to the right of the column under consideration:
        // if there are no such rows or columns, stop since the procedure is finished.
        // Otherwise, carry out the same procedure on the new matrix.

        bound_col += 1;
        bound_row += 1;

        if bound_row >= num_rows || bound_col >= num_cols {
            break;
        }
    }

    for row in 0..num_rows {
        if joltage[row] < 0 {
            joltage[row] *= -1;
            masks_transposed[row] *= i64x16::splat(-1);
        }
    }

    let masks_transposed_again = transpose(&masks_transposed);

    Ok(Machine {
        wiring: masks_transposed_again.iter().map(|x| Mask { val: i16x16::from_array(x.as_array().map(|x| x as i16)) }).take(machine.wiring.len()).collect(),
        joltage: Joltage { val: i16x16::from_array(joltage.as_array().map(|x| x as i16)) },
        indicator_goal: machine.indicator_goal,
        joltage_len: machine.joltage_len
    })
}

fn _gcd<T: std::ops::Add<Output = T> + std::ops::Rem<Output = T> + Copy + Eq + Default>(a: T, b: T) -> T {
    if b == T::default() {
        a
    } else {
        _gcd(b, a % b)
    }
}

fn calc_gcd(row: i64x16, other: i64) -> i64 {
    let gcd: Option<i64> = row.as_array().into_iter().map(|x| *x).reduce(|a, b| {
        if a == 0 {
            if b == 0 {
                0
            } else {
                b
            }
        } else {
            if b == 0 {
                a
            } else {
                _gcd(a, b)
            }
        }
    });

    if let Some(gcd) = gcd {
        _gcd(gcd, other)
    } else {
        other
    }
}

fn remove_empty_rows(machine: &Machine) -> Result<Machine, JsError> {
    let len_masks = machine.wiring.len();
    let last_nonzero_rows = (0..len_masks).rev().find(
        |&idx| machine.wiring[idx].val != i16x16::splat(0)
    ).ok_or_else(|| JsError::new("no nonzero row found"))?;
    let to_len = last_nonzero_rows + 1;

    Ok(Machine {
        wiring: machine.wiring.iter().map(|x| *x).collect(),
        indicator_goal: machine.indicator_goal, // this may be wrong but unused in part 2
        joltage: machine.joltage,
        joltage_len: to_len
    })

}

fn calc_sorted_masks(machine: &Machine) -> Vec<Mask> {
    let mut masks = machine.wiring.clone();

    // sort masks by joltage potential, high to low
    // for a mask: for a joltage: (1 / total joltage)

    masks.sort_by(|a, b| {
        let a_sum = a.val.as_array().iter().enumerate().map(|(joltage_idx, item)| {
            if *item != 0 {
                (1f32 / machine.joltage.val[joltage_idx] as f32)
            } else {
                0f32
            }
        }).sum::<f32>();

        let b_sum = b.val.as_array().iter().enumerate().map(|(joltage_idx, item)| {
            if *item != 0 {
                (1f32 / machine.joltage.val[joltage_idx] as f32)
            } else {
                0f32
            }
        }).sum::<f32>();

        a_sum.partial_cmp(&b_sum).unwrap()
    });

    masks.reverse();

    masks
}


fn calc_constraints(old_constraints: &Option<Vec<Vec<Option<u16>>>>, new_constraints: &mut Vec<Vec<Option<u16>>>,
                    joltage: i16, old_joltages: &Vec<u16>,
                    machine: &Machine, mask_val_and_mask_idx: &Vec<(i16, usize)>, possibility_path: &mut Vec<u16>) -> Result<(), JsError> {
    if mask_val_and_mask_idx.len() == 1 {
        let (val, mask_idx) = mask_val_and_mask_idx[0];
        // shortcut to handle one variable solutions
        if joltage % val == 0 && val > 0 {
            let x = joltage / val;
            match old_constraints {
                Some(_old_constraints) => {
                    for row in _old_constraints {
                        let mut new_row = row.clone();
                        new_row[mask_idx] = Some(x as u16);
                        new_constraints.push(new_row);
                    }
                },
                None => {
                    let mut new_row: Vec<_> = (0..machine.wiring.len()).map(|_| None).collect();
                    new_row[mask_idx] = Some(x as u16);
                    new_constraints.push(new_row);
                }
            }
        } else {
            // no solution
        }

        return Ok(());
    }

    if possibility_path.len() == mask_val_and_mask_idx.len() {
        match old_constraints {
            Some(old_constraints) => {
                // check each constraint row against possibility_path and add if math works out, or if there is a None in the same col as a mask_val_and_mask_idx
                for constraint_row in old_constraints {
                    let mut new_constraint_row = constraint_row.clone();
                    let mut is_match = true;
                    for (idx, (_coef, mask_idx)) in mask_val_and_mask_idx.iter().enumerate() {
                        let possibility = possibility_path[idx];
                        match new_constraint_row[*mask_idx] {
                            Some(constraint_val) => {
                                if constraint_val != possibility {
                                    is_match = false;
                                    break;
                                }
                            },
                            None => {
                                new_constraint_row[*mask_idx] = Some(possibility);
                            }
                        }

                    }

                    if !is_match {
                        continue;
                    }
                    let any_none = new_constraint_row.iter().any(|x| x.is_none());

                    if !any_none && _satisfies_constraint(machine, &new_constraint_row) {
                        new_constraints.push(new_constraint_row);
                    } else if any_none {
                        new_constraints.push(new_constraint_row);
                    }
                }

            },
            None => {
                // starting off with no constraints
                let mut val = 0;

                for (idx, (_coef, mask_idx)) in mask_val_and_mask_idx.iter().enumerate() {
                    let possibility = possibility_path[idx];
                    val += _coef * possibility as i16;
                }
                if val == joltage {
                    let mut new_constraint: Vec<Option<u16>> = (0..machine.wiring.len()).map(|_| None).collect();;

                    for (idx, (_val, mask_idx)) in mask_val_and_mask_idx.iter().enumerate() {
                        new_constraint[*mask_idx] = Some(possibility_path[idx]);
                    }

                    new_constraints.push(new_constraint);
                } else {
                    // else skip
                }
            }

        }
    } else {
        if possibility_path.len() == mask_val_and_mask_idx.len() - 1 {
            // optimization for last column
            let mut running_total = joltage;
            for (idx, possibility) in possibility_path.iter().enumerate() {
                let (_coef, mask_idx) = mask_val_and_mask_idx[idx];
                running_total -= _coef * *possibility as i16;
            }
            let (_coef, last_mask_idx) = mask_val_and_mask_idx[possibility_path.len()];

            let last_possibility = if running_total >= 0 &&  _coef > 0 && running_total % _coef == 0 {
                Some(running_total / _coef)
            } else if running_total <= 0 && _coef < 0 && (running_total * -1) % (_coef * -1) == 0 {
                Some((running_total * -1) / (_coef * -1))
            } else {
                None
            };

            if let Some(last_possibility) = last_possibility {
                possibility_path.push(last_possibility as u16);
                calc_constraints(old_constraints, new_constraints, joltage, old_joltages, machine, &mask_val_and_mask_idx, possibility_path)?;
                possibility_path.pop();
            }
        } else {
            let (coef, mask_idx) = mask_val_and_mask_idx[possibility_path.len()];
            let old_joltage = old_joltages[mask_idx];

            let range: FxHashSet<u16> = match old_constraints {
                Some(old_constraints) => {
                    let mut set: FxHashSet<u16> = FxHashSet::default();

                    let mut constraints_missing = false;
                    for constraint_row in old_constraints {
                        match constraint_row[mask_idx] {
                            Some(_val) => {
                                set.insert(_val);
                            },
                            None => {
                                // no constraint set yet
                                constraints_missing = true;
                                break;
                            }
                        }
                    }

                    // need to fix constraint generating code
                    let range: FxHashSet<u16> = (0..=old_joltage).collect();
                    if constraints_missing {
                        range
                    } else {
                        set.intersection(&range).map(|x| *x).collect()
                    }
                },
                None => {
                    (0..=old_joltage).collect()
                }
            };

            for possibility in range {
                possibility_path.push(possibility);
                calc_constraints(old_constraints, new_constraints, joltage, old_joltages, machine, &mask_val_and_mask_idx, possibility_path)?;
                possibility_path.pop().ok_or_else(|| JsError::new("pop failed"))?;
            }
        }
    }

    Ok(())
}

fn _satisfies_constraint(machine: &Machine, constraint_row: &Vec<Option<u16>>) -> bool {
    for row_num in 0..machine.joltage_len {
        let mut total = 0i16;
        for col_num in 0..machine.wiring.len() {
            let coef = constraint_row[col_num].unwrap() as i16;
            total += coef * machine.wiring[col_num].val[row_num];
        }
        if total != machine.joltage.val[row_num] {
            return false;
        }
    }
    true
}

fn sort_masks(machine: &Machine) -> Result<Machine, JsError> {
    // sort rows by lowest count of coefficients first

    let mut wiring_counts: Vec<_> = (0..machine.joltage_len).map(|row_num| {
        (
            row_num,
            machine.wiring.iter().map(
                |mask| if mask.val[row_num] != 0 { 1 } else { 0 }
            ).sum::<u64>()
        )
    }).collect();
    wiring_counts.sort_by_key(|(_, count)| *count);

    let mut new_wiring: Vec<i16x16> = machine.wiring.iter().map(|_| i16x16::splat(0)).collect();
    let mut new_joltage = i16x16::splat(0);
    for (new_idx, (old_idx, val)) in wiring_counts.iter().enumerate() {
        for (mask_idx, old_mask) in machine.wiring.iter().enumerate() {
            new_wiring[mask_idx][new_idx] = old_mask.val[*old_idx];
        }
        new_joltage[new_idx] = machine.joltage.val[*old_idx];
    }

    Ok(Machine {
        joltage_len: machine.joltage_len,
        joltage: Joltage { val: new_joltage },
        wiring: new_wiring.iter().map(|x| Mask { val: *x }).collect(),
        indicator_goal: machine.indicator_goal,
    })
}

fn calc_min_presses_part_2(old_machine: &Machine) -> Result<u16, JsError> {
    let old_joltages: Vec<u16> = old_machine.wiring.iter().map(|mask| {
        let mut max_joltage = 0;
        for row in 0..old_machine.joltage_len {
            if mask.val[row] != 0 {
                max_joltage = std::cmp::max(max_joltage, old_machine.joltage.val[row] as u16);
            }
        }
        max_joltage
    }).collect();

    let machine_plus_empty_rows = &reduce_rows(old_machine)?;
    let machine_without_empty_rows = &remove_empty_rows(machine_plus_empty_rows)?;
    let machine = &sort_masks(machine_without_empty_rows)?;

    let mut constraints = None;
    for row in 0..machine.joltage_len {
        let joltage = machine.joltage.val[row];
        let mut new_constraints: Vec<Vec<Option<u16>>> = Vec::new();
        let mask_val_and_mask_idx: Vec<(i16, usize)> = machine.wiring.iter().enumerate().filter_map(
            |(idx, mask)| {
                if mask.val[row] != 0 {
                    Some((mask.val[row], idx))
                } else {
                    None
                }
            }
        ).collect();

        let mut possibility_path = Vec::new();
        if !mask_val_and_mask_idx.is_empty() {
            calc_constraints(
                &constraints, &mut new_constraints,
                joltage,
                &old_joltages,
                machine, &mask_val_and_mask_idx, &mut possibility_path
            )?;

            constraints = Some(new_constraints);
        }
    }

    Ok(
        constraints.ok_or_else(
            || JsError::new("no constraints calculated")
        )?.iter().map(
            |items| {
                items.iter().map(|item| item.ok_or_else(
                    || JsError::new("None found in constraints")
                ).unwrap()).sum()
            }
        ).min().ok_or_else(
            || JsError::new("No minimum found")
        )?
    )
}


pub fn day10_part2(input: &str) -> Result<String, JsError> {
    let machines = parse_input(input)?;
    let vals: Vec<_> = machines.iter().map(|machine| {
        calc_min_presses_part_2(machine)
    }).collect::<Result<_, _>>()?;
    let sum = vals.iter().sum::<u16>();
    Ok(sum.to_string())
}
