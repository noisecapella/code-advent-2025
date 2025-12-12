use leptos::wasm_bindgen::JsError;


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Coord {
    x: i128,
    y: i128,
}

fn parse_coords(input: &str) -> Result<Vec<Coord>, JsError> {
    input.lines().map(|line| {
        let mut split = line.split(",");

        let mut parse = || {
            split.next().ok_or_else(
                || JsError::new("Unable to read next number in line")
            ).and_then(
                |numstr| numstr.parse::<i128>().or_else(
                    |err| Err(JsError::new("Unable to parse int"))
                )
            )
        };

        Ok(Coord {
            x: parse()?,
            y: parse()?,
        })
    }).collect()
}

fn calc_area(pair: &(Coord, Coord)) -> i128 {
    let (a, b) = pair;

    (a.x.abs_diff(b.x) + 1) as i128 * (a.y.abs_diff(b.y) + 1) as i128
}

fn coords_into_pairs(coords: &[Coord]) -> Vec<(Coord, Coord)> {
    let mut added = std::collections::HashSet::new();
    let mut pairs = Vec::new();
    for coord_a in coords {
        for coord_b in coords {
            if coord_a != coord_b && !added.contains(coord_b) {
                pairs.push((*coord_a, *coord_b));
            }
        }
        added.insert(coord_a);
    }
    pairs
}


pub fn day9_part1(input: &str) -> Result<String, JsError> {
    let mut coords = parse_coords(input)?;
    let mut pairs = coords_into_pairs(&coords);
    let max_pair = pairs.iter().max_by_key(
        |pair| calc_area(pair)
    ).ok_or_else(
        || JsError::new("Unable to find max pair")
    )?;
    let max_area = calc_area(max_pair);

    Ok(max_area.to_string())
}

fn calc_vert_walls(coords: &Vec<Coord>) -> Result<std::collections::BTreeMap<i128, Vec<VertWall>>, JsError> {
    let mut prev: &Coord = coords.last().ok_or_else(|| JsError::new(&"Could not find last coord"))?;

    let mut walls: std::collections::BTreeMap<i128, Vec<VertWall>> = std::collections::BTreeMap::new();
    for coord in coords {
        if coord.x == prev.x {
            let y1;
            let y2;
            if coord.y < prev.y {
                y1 = coord.y;
                y2 = prev.y;
            } else {
                y1 = prev.y;
                y2 = coord.y;
            }

            walls.entry(coord.x).or_insert_with(|| Vec::new()).push( VertWall { x: coord.x, y1, y2 });
        }

        prev = coord;
    }
    Ok(walls)
}

struct HorzWall {
    y: i128,
    x1: i128,
    x2: i128,
}

struct VertWall {
    x: i128,
    y1: i128,
    y2: i128,
}

fn calc_horz_walls(coords: &Vec<Coord>) -> Result<std::collections::BTreeMap<i128, Vec<HorzWall>>, JsError> {
    let mut prev: &Coord = coords.last().ok_or_else(|| JsError::new(&"Could not find last coord"))?;

    let mut walls: std::collections::BTreeMap<i128, Vec<HorzWall>> = std::collections::BTreeMap::new();
    for coord in coords {
        if coord.y == prev.y {
            let x1;
            let x2;
            if coord.x < prev.x {
                x1 = coord.x;
                x2 = prev.x;
            } else {
                x1 = prev.x;
                x2 = coord.x;
            }

            walls.entry(coord.y).or_insert_with(|| Vec::new()).push( HorzWall { y: coord.y, x1, x2 });
        }

        prev = coord;
    }
    Ok(walls)
}

fn overlap(a_min: i128, a_max: i128, b_min: i128, b_max: i128) -> bool {
    a_max >= b_min && a_min <= b_max
}

pub fn day9_part2(input: &str) -> Result<String, JsError> {
    let coords = parse_coords(input)?;
    let pairs = coords_into_pairs(&coords);

    //
    let global_min_x = coords.iter().map(|coord| coord.x).min().ok_or_else(|| JsError::new("Unable to find min x"))?;

    let vert_walls = calc_vert_walls(&coords)?;
    let horz_walls = calc_horz_walls(&coords)?;

    let is_vert_wall_span = |x, y1, y2| {
        match vert_walls.get(&x) {
            Some(_vec) => {
                _vec.iter().any(|wall| overlap(y1, y2, wall.y1, wall.y2) || overlap(y1, y2, wall.y1, wall.y2))
            },
            None => false
        }
    };

    let overlap_horz_wall = |min_x: i128, min_y: i128, max_x: i128, max_y: i128| {
        if min_x > max_x || min_y > max_y {
            return false;
        }

        for (k, vs) in horz_walls.range(min_y..=max_y) {
            for wall in vs {
                if overlap(min_x, max_x, wall.x1, wall.x2) {
                    return true;
                }
            }

        }
        false


    };
    let overlap_vert_wall = |min_x: i128, min_y: i128, max_x: i128, max_y: i128| {
        if min_x > max_x || min_y > max_y {
            return false;
        }

        for (k, vs) in vert_walls.range(min_x..=max_x) {
            for wall in vs {
                if overlap(min_y, max_y, wall.y1, wall.y2) {
                    return true;
                }
            }

        }
        false
    };

    let areas: Vec<(Coord, Coord)> = pairs.iter().filter_map(
        |pair| {
            let min_x = pair.0.x.min(pair.1.x);
            let max_x = pair.0.x.max(pair.1.x);
            let min_y = pair.0.y.min(pair.1.y);
            let max_y = pair.0.y.max(pair.1.y);

            let target = Coord { x: min_x + 1, y: min_y + 1 };

            let mut outside = true;
            for x in global_min_x..target.x {
                // iterate from left until meeting target
                // y will be assumed to be top, x will be to left of pixel
                let is_wall = is_vert_wall_span(x, target.y - 1, target.y);
                if is_wall {
                    outside = !outside;
                }
            }

            if outside {
                return None;
            }


            if overlap_horz_wall(min_x + 1, min_y + 1 , max_x - 1, max_y - 1 ) {
                return None;
            }
            if overlap_vert_wall(min_x + 1, min_y + 1 , max_x - 1, max_y - 1) {
                return None;
            }

            Some(Ok((Coord { x: min_x, y: min_y }, Coord {x: max_x, y: max_y })))
        }
    ).collect::<Result<Vec<(Coord, Coord)>, JsError>>()?;
    let max_area = areas.iter().map(
        |pair| calc_area(pair)
    ).max().ok_or_else(|| JsError::new("Unable to find max pair in bounds"))?;

    Ok(max_area.to_string())
}

