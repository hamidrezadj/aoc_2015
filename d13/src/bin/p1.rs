use std::{collections::BTreeMap, io};

fn main() {
    let graph: BTreeMap<String, BTreeMap<String, i64>> = io::stdin()
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
            (lhs, rhs, edge)
        })
        .fold(BTreeMap::new(), |mut graph, (lhs, rhs, edge)| {
            graph.entry(lhs).or_default().insert(rhs, edge);
            graph
        });
    let names: Vec<String> = graph.keys().cloned().collect();
    let perms: Vec<Vec<String>> = permutations(names);
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
    println!("{}", max_score.unwrap());
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
