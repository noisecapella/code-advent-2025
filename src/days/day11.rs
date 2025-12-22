use std::collections::hash_map::Entry;
use std::collections::HashMap;
use leptos::wasm_bindgen::JsError;
use leptos::prelude::*;
use leptos::web_sys::console::log_1;

struct Graph {
    node_ids: HashMap<String, u64>,
    links: HashMap<u64, Vec<u64>>,
    back_links: HashMap<u64, Vec<u64>>,
}

fn parse_input(input: &str) -> Result<Graph, JsError> {
    let mut links: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut node_ids: HashMap<String, u64> = HashMap::new();

    let mut get_node_id = |string: &str| {
        let new_val = node_ids.len() as u64;
        let ret = node_ids.entry(string.to_string()).or_insert(new_val);
        *ret
    };

    for line in input.lines() {
        let mut split = line.split(':');
        let key = split.next().ok_or_else(|| JsError::new("couldn't find first" ))?;
        let key_node_id = get_node_id(key);

        let rest = split.next().ok_or_else(|| JsError::new("couldn't find rest" ))?;
        let mut whitespace_split = rest.split_whitespace();

        let mut children = Vec::new();
        for item in whitespace_split {
            let node_id = get_node_id(item);
            children.push(node_id);
        }
        links.insert(key_node_id, children);
    }

    let mut back_links = HashMap::new();
    for (node_id, children) in links.iter() {
        for child in children {
            back_links.entry(*child).or_insert_with(Vec::new).push(*node_id);
        }
    }

    Ok(Graph { node_ids, links, back_links })
}

fn _count_paths_part1(graph: &Graph, current_node: u64, paths_count: &mut HashMap<u64, usize>) -> Result<usize, JsError> {
    let entry = paths_count.entry(current_node);
    if let Entry::Occupied(entry) = entry {
        return Ok(*entry.get());
    }

    let children = graph.links.get(&current_node).ok_or_else(
        || JsError::new("couldn't find children" )
    )?;
    let mut count = 0;
    for child in children {
        count += _count_paths_part1(graph, *child, paths_count)?;
    }

    paths_count.insert(current_node, count);

    Ok(count)
}

pub fn day11_part1(input: &str) -> Result<String, JsError> {
    let graph = parse_input(input)?;

    let end_node = graph.node_ids.get("out").ok_or_else(|| JsError::new("couldn't find end node" ))?;
    let start_node = graph.node_ids.get("you").ok_or_else(|| JsError::new("couldn't find start node" ))?;

    let mut visited_count = HashMap::new();
    visited_count.insert(*end_node, 1);
    _count_paths_part1(&graph, *start_node, &mut visited_count)?;

    let num_paths = visited_count.get(start_node).ok_or_else(
        || JsError::new("couldn't find path" )
    )?;
    Ok(num_paths.to_string())
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Part2Count {
    none: u128,
    dac_only: u128,
    fft_only: u128,
    all: u128
}

fn _count_paths_part2(graph: &Graph, current_node: u64, paths_count: &mut HashMap<u64, Part2Count>) -> Result<Part2Count, JsError> {
    //let current_node_name = graph.node_ids.iter().find(|(_, id)| **id == current_node).ok_or_else(|| JsError::new("couldn't find node name" ))?.0;
    //log_1(&format!("calling with {:?}", current_node_name).into());
    let entry = paths_count.entry(current_node);
    if let Entry::Occupied(entry) = entry {
        //log_1(&format!("Found {:?} = {:?}", current_node_name, entry.get()).into());
        return Ok(*entry.get());
    }

    let children = graph.links.get(&current_node).ok_or_else(
        || JsError::new("couldn't find children" )
    )?;

    let visited_dac = graph.node_ids.get("dac").ok_or_else(|| JsError::new("couldn't find dac" ))? == &current_node;
    let visited_fft =
            graph.node_ids.get("fft").ok_or_else(|| JsError::new("couldn't find fft" ))? == &current_node
    ;

    let mut count = Part2Count { none: 0, dac_only: 0, fft_only: 0, all: 0 };
    for child in children {
        let child_count = _count_paths_part2(graph, *child, paths_count)?;
        count = Part2Count {
            none: count.none + child_count.none,
            all: count.all + child_count.all,
            fft_only: count.fft_only + child_count.fft_only,
            dac_only: count.dac_only + child_count.dac_only,
        };

    }

    if visited_dac {
        if visited_fft {
            count = Part2Count {
                none: 0,
                dac_only: 0,
                fft_only: 0,
                all: (
                    count.all + count.none + count.dac_only + count.fft_only
                )
            };
        } else {
            count = Part2Count {
                none: 0,
                dac_only: (
                    count.none + count.dac_only
                ),
                fft_only: 0,
                all: (
                    count.all + count.fft_only
                )
            };
        }
    } else {
        if visited_fft {
            count = Part2Count {
                none: 0,
                dac_only: 0,
                fft_only: (
                    count.none + count.fft_only
                ),
                all: (
                    count.all + count.dac_only
                )
            };

        } else {
            // counts stay as they are
        }
    }
    //log_1(&format!("Inserting {:?} = {:?}, visited_dac = {}, visited_fft = {}", current_node_name, count, visited_dac, visited_fft).into());
    paths_count.insert(current_node, count);

    Ok(count)
}

pub fn day11_part2(input: &str) -> Result<String, JsError> {
    let graph = parse_input(input)?;

    let end_node = graph.node_ids.get("out").ok_or_else(|| JsError::new("couldn't find end node" ))?;
    let start_node = graph.node_ids.get("svr").ok_or_else(|| JsError::new("couldn't find start node" ))?;

    let mut visited_count = HashMap::new();
    visited_count.insert(*end_node, Part2Count { none: 1, all: 0, fft_only: 0, dac_only: 0 } );
    _count_paths_part2(&graph, *start_node, &mut visited_count)?;

    let num_paths = visited_count.get(start_node).ok_or_else(
        || JsError::new("couldn't find path" )
    )?;
    Ok(format!("{:?}", num_paths))
}
