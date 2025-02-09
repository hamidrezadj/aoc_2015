use std::{collections::BTreeSet, io};

fn main() {
    let input = io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .next()
        .expect("No input");
    let mut position_santa: (i64, i64) = (0, 0);
    let mut position_robo_santa: (i64, i64) = (0, 0);
    let mut visited_set = BTreeSet::from([(0, 0)]);
    for (i, ch) in input.chars().enumerate() {
        let position = if i % 2 == 0 {
            &mut position_santa
        } else {
            &mut position_robo_santa
        };
        move_position(ch, position);
        visited_set.insert(*position);
    }
    let answer = visited_set.len();
    println!("{}", answer);
}

fn move_position(ch: char, position: &mut (i64, i64)) {
    match ch {
        '^' => *position = (position.0 - 1, position.1),
        '>' => *position = (position.0, position.1 + 1),
        'v' => *position = (position.0 + 1, position.1),
        '<' => *position = (position.0, position.1 - 1),
        _ => panic!("Invalid character"),
    }
}
