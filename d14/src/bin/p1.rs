use std::io;

fn main() {
    let time = 2503;
    let score = io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error."))
        .map(|line| {
            let mut splits = line.split_whitespace();
            let name = splits.next().expect("Not enough words in line.").to_owned();
            let speed = splits
                .nth(2)
                .expect("Not enough words in line.")
                .parse::<u64>()
                .expect("Bad speed integer.");
            let run_time = splits
                .nth(2)
                .expect("Not enough words in line.")
                .parse::<u64>()
                .expect("Bad run time integer.");
            let rest_time = splits
                .nth(6)
                .expect("Not enough words in line.")
                .parse::<u64>()
                .expect("Bad rest time integer.");
            (name, speed, run_time, rest_time)
        })
        .map(|(_name, speed, run_time, rest_time)| {
            let cycles = time / (run_time + rest_time);
            let remainder = time % (run_time + rest_time);
            cycles * speed * run_time
                + (if remainder > run_time {
                    run_time
                } else {
                    remainder
                }) * speed
        })
        .max()
        .expect("Empty input.");
    println!("{}", score);
}
