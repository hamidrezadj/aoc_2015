use std::io;

#[derive(Debug)]
enum Operation {
    Off,
    On,
    Toggle,
}

#[derive(Debug)]
struct Point {
    i: usize,
    j: usize,
}

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    start: Point,
    end: Point,
}

fn main() {
    let score = io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            let mut splits = line.split_whitespace();
            let first_word = splits.next().expect("Empty line");
            let operation = match first_word {
                "turn" => {
                    let second_word = splits.next().expect("Invalid pattern");
                    match second_word {
                        "off" => Operation::Off,
                        "on" => Operation::On,
                        _ => panic!("Invalid pattern"),
                    }
                }
                "toggle" => Operation::Toggle,
                _ => panic!("Invalid pattern"),
            };
            let start: Vec<usize> = splits
                .next()
                .expect("Invalid pattern")
                .split(',')
                .map(|num_str| num_str.parse::<usize>().expect("Invalid integer"))
                .collect();
            let end: Vec<usize> = splits
                .nth(1)
                .expect("Invalid pattern")
                .split(',')
                .map(|num_str| num_str.parse::<usize>().expect("Invalid integer"))
                .collect();
            assert_eq!(start.len(), 2);
            assert_eq!(end.len(), 2);
            let start = Point {
                i: start[0],
                j: start[1],
            };
            let end = Point {
                i: end[0],
                j: end[1],
            };
            assert!(start.i < 1000 && start.j < 1000 && end.i < 1000 && end.j < 1000);
            assert!(start.i < end.i);
            assert!(start.j < end.j);
            Instruction {
                operation,
                start,
                end,
            }
        })
        .fold(
            vec![vec![0u64; 1000]; 1000],
            |mut light_map, instruction| {
                for row in light_map
                    .iter_mut()
                    .take(instruction.end.i + 1)
                    .skip(instruction.start.i)
                {
                    for light in row
                        .iter_mut()
                        .take(instruction.end.j + 1)
                        .skip(instruction.start.j)
                    {
                        match instruction.operation {
                            Operation::Off => *light = light.saturating_sub(1),
                            Operation::On => *light = light.saturating_add(1),
                            Operation::Toggle => *light = light.saturating_add(2),
                        }
                    }
                }
                light_map
            },
        )
        .iter()
        .flatten()
        .sum::<u64>();
    println!("{}", score);
}
