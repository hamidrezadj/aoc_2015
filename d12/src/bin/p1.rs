use std::io;

fn main() {
    let input = io::stdin()
        .lines()
        .next()
        .expect("Empty input")
        .expect("Stdin error");
    let mut buffer = String::new();
    let mut score = 0;
    for (i, ch) in input.chars().enumerate() {
        match ch {
            ch @ '0'..='9' if i == input.len() - 1 => {
                buffer.push(ch);
                score += buffer.parse::<i64>().expect("Bad integer");
            }
            ch @ '0'..='9' => {
                buffer.push(ch);
            }
            '-' => {
                buffer.push('-');
            }
            _ if buffer.is_empty() => continue,
            _ => {
                score += buffer.parse::<i64>().expect("Bad integer");
                buffer.clear();
            }
        }
    }
    println!("{}", score);
}
