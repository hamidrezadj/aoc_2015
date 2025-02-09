use std::{collections::BTreeSet, io};

fn main() {
    let input = io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .next()
        .expect("No input");
    let mut position: (i64, i64) = (0, 0);
    let mut visited_set = BTreeSet::from([position]);
    for ch in input.chars() {
        match ch {
            '^' => position = (position.0 - 1, position.1),
            '>' => position = (position.0, position.1 + 1),
            'v' => position = (position.0 + 1, position.1),
            '<' => position = (position.0, position.1 - 1),
            _ => panic!("Invalid character"),
        }
        visited_set.insert(position);
    }
    let answer = visited_set.len();
    println!("{}", answer);
}
