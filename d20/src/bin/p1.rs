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
    let input = input / 10;
    let output = (1..).find(|i| sum_of_divisors_of(*i) >= input).unwrap();
    println!("{}", output);
}

fn sum_of_divisors_of(n: u64) -> u64 {
    (1..=n.isqrt())
        .filter(|d| n % d == 0)
        .flat_map(|d| [d, n / d])
        .collect::<HashSet<u64>>()
        .into_iter()
        .sum()
}
