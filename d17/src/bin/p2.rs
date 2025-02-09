use std::{collections::BTreeMap, env, io};

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
    assert!(
        volumes.len() <= 64,
        "More than 64 containers is not supported"
    );
    // counts is a map from volume count to
    // the number of valid permutations for that volume count.
    let counts: BTreeMap<usize, usize> = (0..2u64.pow(volumes.len() as u32))
        .filter_map(|permutation| {
            let is_valid_permutation = (0..volumes.len())
                .filter(|idx| (1 << idx) & permutation > 0)
                .map(|idx| volumes[idx])
                .sum::<u64>()
                .eq(&target_volume);
            if !is_valid_permutation {
                return None;
            }
            let volume_count = (0..volumes.len())
                .filter(|idx| (1 << idx) & permutation > 0)
                .count();
            Some(volume_count)
        })
        .fold(BTreeMap::new(), |mut map, volume_count| {
            *map.entry(volume_count).or_insert(0) += 1;
            map
        });
    let min_volume_count = counts.keys().min().expect("No valid permutation");
    let count = counts.get(min_volume_count).unwrap();
    println!("{}", count);
}
