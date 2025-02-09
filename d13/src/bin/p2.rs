use std::{collections::BTreeMap, io};

type Graph = BTreeMap<String, BTreeMap<String, i64>>;

fn main() {
    let graph = read_input();
    let graph = add_me(graph);
    let names: Vec<String> = graph.keys().cloned().collect();
    let perms: Vec<Vec<String>> = permutations(names);
    let score = max_score(perms, &graph);
    println!("{}", score.unwrap());
}

fn add_me(mut graph: Graph) -> Graph {
    for node in graph.values_mut() {
        node.insert(String::from("Me"), 0);
    }
    let names: Vec<String> = graph.keys().cloned().collect();
    graph.insert(String::from("Me"), BTreeMap::new());
    let me_node = graph.get_mut(&String::from("Me")).unwrap();
    for name in names {
        me_node.insert(name.clone(), 0);
    }
    graph
}

fn read_input() -> Graph {
    let graph: Graph = io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error."))
        .map(|line| {
            let mut splits = line.split_whitespace();
            let lhs = splits.next().expect("Empty input line.");
            let lhs = lhs.to_owned();
            let sign = splits.nth(1).expect("Not enough words in line.");
            let sign = match sign {
                "gain" => 1,
                "lose" => -1,
                _ => panic!("Bad sign: gain and lose are allowed only."),
            };
            let size = splits
                .next()
                .expect("Not enough words in line.")
                .parse::<i64>()
                .expect("Bad intger.");
            assert!(size >= 0, "Negative favorability size.");
            let edge = size * sign;
            let rhs = splits.nth(6).expect("Not enough words in line.");
            assert_eq!(rhs.chars().last().unwrap(), '.');
            let (rhs, _dot) = rhs.split_at(rhs.len() - 1);
            let rhs = rhs.to_owned();
            assert_ne!(lhs, "Me");
            assert_ne!(rhs, "Me");
            (lhs, rhs, edge)
        })
        .fold(BTreeMap::new(), |mut graph, (lhs, rhs, edge)| {
            graph.entry(lhs).or_default().insert(rhs, edge);
            graph
        });
    graph
}

fn max_score(perms: Vec<Vec<String>>, graph: &Graph) -> Option<i64> {
    let mut max_score: Option<i64> = None;
    for perm in perms {
        let mut score = 0;
        for i in 0..perm.len() {
            let name = &perm[i];
            let left_name = &perm[if i > 0 { i - 1 } else { perm.len() - 1 }];
            let right_name = &perm[if i < perm.len() - 1 { i + 1 } else { 0 }];
            let node = &graph.get(name).unwrap();
            score += node.get(left_name).unwrap();
            score += node.get(right_name).unwrap();
        }
        if max_score.is_none() || max_score.unwrap() < score {
            max_score = Some(score);
        }
    }
    max_score
}

fn permutations(names: Vec<String>) -> Vec<Vec<String>> {
    if names.len() < 4 {
        return vec![names];
    }
    let (seed, rest) = names.split_at(3);
    let seed = seed.to_owned();
    let rest = rest.to_owned();
    let mut perms = vec![seed];
    for name in rest {
        let mut next_gen_perms = Vec::new();
        for perm in perms {
            for i in 0..perm.len() {
                let mut new_perm = perm.clone();
                new_perm.insert(i, name.clone());
                next_gen_perms.push(new_perm);
            }
        }
        perms = next_gen_perms;
    }
    perms
}
