use std::io;

fn main() {
    let input = io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .next()
        .unwrap();
    let mut floor = 0;
    for (i, ch) in input.chars().enumerate() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Invalid character"),
        }
        if floor == -1 {
            println!("{}", i + 1);
            return;
        }
    }
}
