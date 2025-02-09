use std::{collections::BinaryHeap, io};

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Reindeer {
    name: String,
    speed: u64,
    run_time: u64,
    rest_time: u64,
}
#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Record {
    distance: u64,
    score: u64,
    reindeer: Reindeer,
}

fn main() {
    let mut score_records: BinaryHeap<Record> = io::stdin()
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
            Reindeer {
                name,
                speed,
                run_time,
                rest_time,
            }
        })
        .map(|reindeer| Record {
            distance: 0,
            score: 0,
            reindeer,
        })
        .collect();
    assert!(!score_records.is_empty());
    let time = 2503;
    for t in 1..=time {
        let mut new_score_records = BinaryHeap::new();
        let mut current_max_distance = None;
        while let Some(mut record) = score_records.pop() {
            if current_max_distance.is_none() {
                current_max_distance = Some(record.distance);
            }
            // This has the undesirable consequence of giving a
            // bias of size one to every score.
            if record.distance == current_max_distance.unwrap() {
                record.score += 1;
            }
            let reindeer = &record.reindeer;
            let cycles = t / (reindeer.run_time + reindeer.rest_time);
            let remainder = t % (reindeer.run_time + reindeer.rest_time);
            let distance = cycles * reindeer.speed * reindeer.run_time
                + reindeer.speed * remainder.clamp(0, reindeer.run_time);
            record.distance = distance;
            new_score_records.push(record);
        }
        score_records = new_score_records;
    }
    let mut final_scores = BinaryHeap::new();
    let mut current_max_distance = None;
    while let Some(mut record) = score_records.pop() {
        if current_max_distance.is_none() {
            current_max_distance = Some(record.distance);
        }
        // Removing the undesirable bias of size one.
        if record.distance != current_max_distance.unwrap() {
            record.score -= 1;
        }
        final_scores.push(record.score);
    }
    println!("{}", final_scores.pop().unwrap());
}
