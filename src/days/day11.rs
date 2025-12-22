use std::collections::hash_map::Entry;
use std::collections::HashMap;
use leptos::wasm_bindgen::JsError;
use leptos::prelude::*;

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

fn _count_paths(graph: &Graph, current_node: u64, paths_count: &mut HashMap<u64, usize>) -> Result<usize, JsError> {
    let entry = paths_count.entry(current_node);
    if let Entry::Occupied(entry) = entry {
        return Ok(*entry.get());
    }

    let children = graph.links.get(&current_node).ok_or_else(
        || JsError::new("couldn't find children" )
    )?;
    let mut count = 0;
    for child in children {
        count += _count_paths(graph, *child, paths_count)?;
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
    _count_paths(&graph, *start_node, &mut visited_count)?;

    let num_paths = visited_count.get(start_node).ok_or_else(
        || JsError::new("couldn't find path" )
    )?;
    Ok(num_paths.to_string())
}

pub fn day11_part2(input: &str) -> Result<String, JsError> {
    Ok("".to_string())
}
