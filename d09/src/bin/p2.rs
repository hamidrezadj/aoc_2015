use std::{collections::BTreeMap, io};

use itertools::Itertools;

fn main() {
    let graph: BTreeMap<String, BTreeMap<String, u64>> = io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            let (nodes_str, distance_str) = line
                .split_once(" = ")
                .unwrap_or_else(|| panic!("Invalid pattern: no equal sign: {}", line));
            let (node_a, node_b) = nodes_str
                .split_once(" to ")
                .unwrap_or_else(|| panic!("Invalid pattern: no 'to' key word: {}", nodes_str));
            let distance = distance_str.parse::<u64>().unwrap_or_else(|_| {
                panic!("Invalid pattern: Bad distance integer: {}", distance_str)
            });
            assert_ne!(
                node_a, node_b,
                "Invalid pattern: Each city has to go to a different one: {}",
                node_a,
            );
            (node_a.to_owned(), node_b.to_owned(), distance)
        })
        .fold(BTreeMap::new(), |mut graph, (node_a, node_b, distance)| {
            if graph
                .entry(node_a.clone())
                .or_default()
                .insert(node_b.clone(), distance)
                .is_some()
            {
                panic!("Duplicate entry: {}, {}", node_a, node_b);
            }
            if graph
                .entry(node_b.clone())
                .or_default()
                .insert(node_a.clone(), distance)
                .is_some()
            {
                panic!("Duplicate entry: {}, {}", node_a, node_b);
            }
            graph
        });
    assert!(!graph.is_empty(), "Empty graph (input)");

    // This is not particularly performant
    // because it doesn't deduplicate the routes
    // eg; [a, b, c], [c, b, a]
    let mut max_length = None;
    for mut permutation in graph.keys().permutations(graph.len()) {
        let mut current_node: &str = permutation.pop().unwrap();
        let mut length = 0;
        while let Some(next_node) = permutation.pop() {
            length += graph.get(current_node).unwrap().get(next_node).unwrap();
            current_node = next_node;
        }
        if max_length.is_none() || max_length.unwrap() < length {
            max_length = Some(length);
        }
    }
    println!("{}", max_length.unwrap());
}
