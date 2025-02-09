use std::{env, io};

fn main() {
    let volumes: Vec<u64> = io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| line.parse::<u64>().expect("Bad integer"))
        .collect();
    let default_target_volume = 150;
    let target_volume = env::var("TARGET_VOLUME")
        .ok()
        .and_then(|target_volume| target_volume.parse::<u64>().ok())
        .unwrap_or(default_target_volume);
    assert!(volumes.len() <= 64);
    let count = (0..2u64.pow(volumes.len() as u32))
        .filter(|permutation| {
            (0..volumes.len())
                .filter(|idx| (1 << idx) & permutation > 0)
                .map(|idx| volumes[idx])
                .sum::<u64>()
                .eq(&target_volume)
        })
        .count();
    println!("{}", count);
}
