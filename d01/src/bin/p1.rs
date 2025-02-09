use std::io;

fn main() {
    let input = io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .next()
        .unwrap();
    let positive_instructions_count = input.chars().filter(|ch| *ch == '(').count();
    let negative_instructions_count = input.chars().filter(|ch| *ch == ')').count();
    let answer = positive_instructions_count as isize - negative_instructions_count as isize;
    println!("{}", answer);
}
