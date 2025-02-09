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
        .map(|dimensions| {
            let first_rectangle_area = dimensions[0] * dimensions[1];
            let second_rectangle_area = dimensions[0] * dimensions[2];
            let third_rectangle_area = dimensions[1] * dimensions[2];
            let min_area = [
                first_rectangle_area,
                second_rectangle_area,
                third_rectangle_area,
            ]
            .into_iter()
            .min()
            .unwrap();
            first_rectangle_area * 2
                + second_rectangle_area * 2
                + third_rectangle_area * 2
                + min_area
        })
        .sum::<u64>();
    println!("{}", answer);
}
