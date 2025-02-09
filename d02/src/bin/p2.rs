use std::io;

fn main() {
    let answer = io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            line.split('x')
                .map(|split| split.parse::<u64>().expect("Bad integer"))
                .collect::<Vec<u64>>()
        })
        .map(|mut dimensions| {
            dimensions.sort();
            let smallest_perimeter = 2 * (dimensions[0] + dimensions[1]);
            let additional_ribbon = dimensions.iter().product::<u64>();
            smallest_perimeter + additional_ribbon
        })
        .sum::<u64>();
    println!("{}", answer);
}
