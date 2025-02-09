use std::io;

fn main() {
    let mut input = io::stdin()
        .lines()
        .next()
        .expect("Empty input")
        .expect("Stdin error");
    assert!(!input.is_empty(), "Empty input line");
    for _ in 0..50 {
        let mut last_ch = None;
        let mut last_ch_count = 0;
        let mut output: String = String::new();
        for ch in input.chars() {
            match (last_ch.is_some(), last_ch.is_some_and(|lch| lch == ch)) {
                (true, true) => {
                    last_ch_count += 1;
                }
                (true, false) => {
                    output.push_str(&last_ch_count.to_string());
                    output.push(last_ch.unwrap());
                    last_ch = Some(ch);
                    last_ch_count = 1;
                }
                (false, _) => {
                    last_ch = Some(ch);
                    last_ch_count += 1;
                }
            }
        }
        output.push_str(&last_ch_count.to_string());
        output.push(last_ch.unwrap());
        input = output;
    }
    println!("{}", input.len());
}
