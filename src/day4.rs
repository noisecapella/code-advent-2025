use leptos::wasm_bindgen::JsError;

struct Board {
    items: Vec<char>,
    num_rows: usize,
    num_cols: usize,
}

impl Board {
    fn get(&self, c: i32, r: i32) -> Option<char> {
        let num_cols = self.num_cols as i32;
        let num_rows = self.num_rows as i32;

        if c < num_cols && r < num_rows && c >= 0 && r >= 0 {
            let idx = (c as usize) + (r as usize * self.num_cols);
            let item = self.items.get(idx);
            item.and_then(|c| Some(*c))
        } else {
            None
        }
    }

    fn get_mut(&mut self, c: i32, r: i32) -> Result<&mut char, JsError> {
        let num_cols = self.num_cols as i32;
        let num_rows = self.num_rows as i32;

        if c < num_cols && r < num_rows && c >= 0 && r >= 0 {
            let idx = (c as usize) + (r as usize * self.num_cols);
            self.items.get_mut(idx).ok_or_else(|| JsError::new("out of bounds"))
        } else {
            Err(JsError::new("out of bounds"))
        }
    }
}

fn read_board(input: &str) -> Result<Board, JsError> {
    let first_line: &str = input.lines().next().ok_or_else(
        || JsError::new("Could not read first line")
    )?;
    let num_cols = first_line.len();
    if num_cols == 0 {
        return Err(JsError::new("First line is empty"));
    }

    let mut items: Vec<char> = Vec::new();
    let mut num_rows = 0;
    for line in input.lines() {
        if line.len() == 0 {
            continue;
        }

        if line.len() != num_cols {
            return Err(JsError::new("Line length does not match first line"));
        }

        for char in line.chars() {
            items.push(char);
        }
        num_rows += 1;
    }

    Ok(Board {
        items,
        num_rows,
        num_cols,
    })
}

fn is_accessible(board: &Board, c: i32, r: i32) -> Result<bool, JsError> {
    let center = board.get(c, r);
    match center {
        Some(_center) => {
            if _center != '@' {
                return Ok(false);
            }
        }
        None => {
            return Err(
                JsError::new("unexpected error, checking empty space")
            );
        }
    }

    let mut count = 0;
    for check_r in r - 1..=r + 1 {
        for check_c in c - 1..=c + 1 {
            if check_r == r && check_c == c {
                continue;
            }
            let item = board.get(check_c, check_r);
            match item {
                Some(_item) => {
                    if _item == '@' {
                        count += 1;
                    }
                },
                None => {

                }
            }
        }
    }
    Ok(count < 4)
}

fn count_accessible(board: &Board) -> Result<usize, JsError> {
    let mut count = 0;
    for r in 0..board.num_rows {
        for c in 0..board.num_cols {
            if is_accessible(&board, c as i32, r as i32)? {
                count += 1;
            }
        }
    }
    Ok(count)
}

pub fn day4_part1(input: &str) -> Result<String, JsError> {
    let board = read_board(input)?;

    let count = count_accessible(&board)?;

    Ok(count.to_string())
}

pub fn day4_part2(input: &str) -> Result<String, JsError> {
    let mut board = read_board(input)?;

    let mut count = 0;
    while count_accessible(&board)? > 0 {
        for r in 0..board.num_rows {
            for c in 0..board.num_cols {
                if is_accessible(&board, c as i32, r as i32)? {
                    let ref_item = board.get_mut(c as i32, r as i32)?;
                    *ref_item = 'x';
                    count += 1;
                }
            }
        }
    }

    Ok(count.to_string())
}
