use std::collections::HashSet;

fn main() {
    let input: u64 = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .next()
        .map(|line| {
            line.parse()
                .expect("Input is not an unsigned integer that fits in 64 bits")
        })
        .expect("Empty input");
    let output = (1..).find(|i| house_gift_count(*i) * 11 >= input).unwrap();
    println!("{}", output);
}

fn house_gift_count(n: u64) -> u64 {
    (1..=n.isqrt())
        .filter(|divisor| n % divisor == 0)
        .flat_map(|divisor| {
            let mut elves_numbers = Vec::new();
            if n / divisor <= 50 {
                elves_numbers.push(divisor);
            }
            if divisor <= 50 {
                elves_numbers.push(n / divisor);
            }
            elves_numbers
        })
        .collect::<HashSet<u64>>()
        .into_iter()
        .sum()
}
