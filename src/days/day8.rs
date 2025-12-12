use leptos::wasm_bindgen::JsError;


#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
struct Coord {
    x: u32,
    y: u32,
    z: u32,
}

fn parse_coords(input: &str) -> Result<Vec<Coord>, JsError> {
    input.lines().map(|line| {
        let mut split = line.split(",");

        let mut parse = || {
            split.next().ok_or_else(
                || JsError::new("Unable to read")
            ).and_then(
                |numstr| str::parse::<u32>(numstr).or_else(
                    |err| Err(JsError::new("unable to parse int"))
                )
            ).or_else(
                |err| Err(JsError::new("Unable to parse int"))
            )
        };

        Ok(Coord {
            x: parse()?,
            y: parse()?,
            z: parse()?,
        })
    }).collect()
}

fn distance(pair: &(Coord, Coord)) -> f32 {
    let (a, b) = pair;
    let ax = a.x as f32;
    let ay = a.y as f32;
    let az = a.z as f32;
    let bx = b.x as f32;
    let by = b.y as f32;
    let bz = b.z as f32;

    (ax - bx).powi(2) + (ay - by).powi(2) + (az - bz).powi(2)
}

fn sort_coords_into_pairs(coords: &[Coord]) -> Vec<(Coord, Coord)> {
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
    pairs.sort_by(|coord_pair_a, coord_pair_b| {
        let distance_a = distance(coord_pair_a);
        let distance_b = distance(coord_pair_b);
        if distance_a < distance_b {
            std::cmp::Ordering::Less
        } else if distance_a > distance_b {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    pairs
}


pub fn day8_part1(input: &str) -> Result<String, JsError> {
    let coords = parse_coords(input)?;
    let sorted_pairs = sort_coords_into_pairs(&coords);

    let mut circuits: std::collections::HashMap<Coord, usize> = coords.iter().enumerate().map(|(idx, coord)| {
        (*coord, idx)
    }).collect();


    for (coord_a, coord_b) in sorted_pairs.iter().take(1000) {
        let circuit_a = *circuits.get(&coord_a).ok_or_else(
            || JsError::new("Unable to find coord at index for a")
        )?;
        let circuit_b = *circuits.get(&coord_b).ok_or_else(
            || JsError::new("Unable to find coord at index for b")
        )?;

        for (coord, circuit) in circuits.iter_mut() {
            if *circuit == circuit_b {
                *circuit = circuit_a;
            }
        }
        //break;
    }

    let circuit_counts: std::collections::HashMap<usize, usize> = circuits.into_iter().fold(
        std::collections::HashMap::new(),
        |mut acc, (coord, circuit)| {
            match acc.entry(circuit) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    *entry.get_mut() += 1;
                },
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(1);
                }
            };
            acc
        }
    );


    let mut sorted_counts: Vec<usize> = circuit_counts.values().map(|size| *size).collect();
    sorted_counts.sort();
    sorted_counts.reverse();

    let mut sum: usize = sorted_counts.iter().take(3).product();

    Ok(sum.to_string())
}

pub fn day8_part2(input: &str) -> Result<String, JsError> {
    let coords = parse_coords(input)?;
    let sorted_pairs = sort_coords_into_pairs(&coords);

    let mut circuits: std::collections::HashMap<Coord, usize> = coords.iter().enumerate().map(|(idx, coord)| {
        (*coord, idx)
    }).collect();


    for (coord_a, coord_b) in sorted_pairs {
        let circuit_a = *circuits.get(&coord_a).ok_or_else(
            || JsError::new("Unable to find coord at index for a")
        )?;
        let circuit_b = *circuits.get(&coord_b).ok_or_else(
            || JsError::new("Unable to find coord at index for b")
        )?;

        for (coord, circuit) in circuits.iter_mut() {
            if *circuit == circuit_b {
                *circuit = circuit_a;
            }
        }
        //break;
        let count_set: std::collections::HashSet<usize> = circuits.values().map(|v| *v).collect();
        if count_set.len() == 1 {
            return Ok((coord_a.x * coord_b.x).to_string());
        }
    }


    Err(JsError::new("Unable to find solution"))

}
