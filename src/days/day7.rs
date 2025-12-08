use leptos::wasm_bindgen::JsError;


#[derive(Clone)]
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

    fn get_mut(&mut self, c: i32, r: i32) -> Option<&mut char> {
        let num_cols = self.num_cols as i32;
        let num_rows = self.num_rows as i32;

        if c < num_cols && r < num_rows && c >= 0 && r >= 0 {
            let idx = (c as usize) + (r as usize * self.num_cols);
            self.items.get_mut(idx)
        } else {
            None
        }
    }
}

fn parse_board(input: &str) -> Result<Board, JsError> {
    let grid: Vec<char> = input.lines().flat_map(|line| {
        line.chars()
    }).collect();

    let mut num_cols = None;
    for line in input.lines() {
        if let Some(_num_cols) = num_cols {
            if _num_cols != line.len() {
                return Err(JsError::new("number of columns doesn't match"));
            }
        } else {
            num_cols = Some(line.len());
        }
    }

    match num_cols {
        Some(num_cols) => {
            Ok(Board {
                items: grid,
                num_rows: input.lines().count(),
                num_cols: num_cols,
            })
        },
        None => return Err(JsError::new("no columns found")),
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Position {
    row: usize,
    col: usize
}

fn calc_splits_part1(board: &Board) -> Result<usize, JsError> {
    let mut _board = board.clone();
    let mut num_splits = 0;

    for row in 0..board.num_rows {
        for col in 0..board.num_cols {
            match _board.get(col as i32, row as i32) {
                Some('S' | '|') => {
                    match _board.get_mut(col as i32, (row + 1) as i32) {
                        Some('^') => {
                            num_splits += 1;
                            if let Some(_to_mut) = _board.get_mut((col - 1) as i32, (row + 1) as i32) {
                                if *_to_mut != '.' && *_to_mut != '|' {
                                    return Err(JsError::new("not an empty space"));
                                }
                                *_to_mut = '|';
                            }

                            if let Some(_to_mut) = _board.get_mut((col + 1) as i32, (row + 1) as i32) {
                                if *_to_mut != '.' && *_to_mut != '|' {
                                    return Err(JsError::new("not an empty space"));
                                }
                                *_to_mut = '|';
                            }
                        },
                        Some(_next_piece) => {
                            if *_next_piece == '.' {
                                *_next_piece = '|';
                            } else if *_next_piece == '|' {

                            } else {
                                return Err(JsError::new("unexpected piece"));
                            }
                        },
                        None => {
                            // end of board
                        }
                    }
                },
                Some('.' | '^') => {

                },
                Some(_) => {
                    return Err(JsError::new("unexpected character in current row"));
                },
                None => {
                    return Err(JsError::new("unexpected end of board"));
                }
            }
        }
    }

    Ok(num_splits)
}


pub fn day7_part1(input: &str) -> Result<String, JsError> {
    let board = parse_board(input)?;
    let num_splits = calc_splits_part1(&board)?;

    Ok(num_splits.to_string())
}

fn calc_start(board: &Board) -> Option<Position> {
    for row in 0..board.num_rows {
        for col in 0..board.num_cols {
            match board.get(col as i32, row as i32) {
                Some('S') => {
                    return Some(Position { row, col });
                },
                _ => {}
            }
        }
    }
    None
}

fn calc_splits_part2(board: &Board, position: Position, known: &mut std::collections::HashMap<Position, u128>) -> Result<u128, JsError> {
    match board.get(position.col as i32, (position.row + 1) as i32) {
        Some('^') => {
            let left_position = Position { row: position.row + 1, col: position.col - 1 };
            let right_position = Position { row: position.row + 1, col: position.col + 1 };

            let left = if let Some(left_item) = known.get(&left_position) {
                *left_item
            } else {
                let left_item = calc_splits_part2(
                    board,
                    left_position,
                    known
                )?;
                known.insert(left_position, left_item);
                left_item
            };

            let right = if let Some(right_item) = known.get(&right_position) {
                *right_item
            } else {
                let right_item = calc_splits_part2(
                    board,
                    right_position,
                    known
                )?;
                known.insert(right_position, right_item);
                right_item
            };

            Ok(1 + left + right)
        },
        Some('.') => {
            let center_position = Position { row: position.row + 1, col: position.col };

            let center = if let Some(center_item) = known.get(&center_position) {
                *center_item
            } else {
                let center_item = calc_splits_part2(
                    board,
                    center_position,
                    known
                )?;
                known.insert(center_position, center_item);
                center_item
            };

            Ok(center)
        },
        Some(_) => {
            Err(JsError::new("unexpected character in next row"))
        }
        None => {
            // end of board
            Ok(0)
        }
    }
}
pub fn day7_part2(input: &str) -> Result<String, JsError> {
    let board = parse_board(input)?;

    let start = calc_start(&board).ok_or_else(
        || JsError::new("No start found")
    )?;
    let mut known: std::collections::HashMap<Position, u128> = std::collections::HashMap::new();
    let num_splits = 1 + calc_splits_part2(&board, start, &mut known)?;
    Ok(num_splits.to_string())
}