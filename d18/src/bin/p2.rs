use std::{env, io};

fn main() {
    let mut lights: Vec<Vec<u32>> = io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!("Bad character"),
                })
                .collect()
        })
        .collect();
    assert!(!lights.is_empty(), "Empty input");
    assert!(!lights[0].is_empty(), "Empty first row");
    assert!(
        lights.iter().all(|row| row.len() == lights[0].len()),
        "Uneven rows"
    );
    let max_i = lights.len() - 1;
    let max_j = lights[0].len() - 1;
    lights[0][0] = 1;
    lights[0][max_j] = 1;
    lights[max_i][0] = 1;
    lights[max_i][max_j] = 1;
    let default_iteration_count = 100;
    let iteration_count = env::var("ITERATION_COUNT")
        .ok()
        .and_then(|ic| ic.parse::<u64>().ok())
        .unwrap_or(default_iteration_count);
    for _ in 0..iteration_count {
        lights = lights
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, light)| {
                        let up = || lights[i - 1][j];
                        let up_right = || lights[i - 1][j + 1];
                        let right = || lights[i][j + 1];
                        let down_right = || lights[i + 1][j + 1];
                        let down = || lights[i + 1][j];
                        let down_left = || lights[i + 1][j - 1];
                        let left = || lights[i][j - 1];
                        let up_left = || lights[i - 1][j - 1];
                        let surrounding_lights_count = match (i, j) {
                            (0, 0) => 3,
                            (0, j) if j == max_j => 3,
                            (i, 0) if i == max_i => 3,
                            (i, j) if i == max_i && j == max_j => 3,
                            (0, _) => right() + down_right() + down() + down_left() + left(),
                            (_, 0) => up() + up_right() + right() + down_right() + down(),
                            (i, _) if i == max_i => {
                                up() + up_right() + right() + left() + up_left()
                            }
                            (_, j) if j == max_j => {
                                up() + down() + down_left() + left() + up_left()
                            }
                            (_, _) => {
                                up() + up_right()
                                    + right()
                                    + down_right()
                                    + down()
                                    + down_left()
                                    + left()
                                    + up_left()
                            }
                        };
                        match (i, j, light) {
                            (0, 0, _) => 1,
                            (0, j, _) if j == max_j => 1,
                            (i, 0, _) if i == max_i => 1,
                            (i, j, _) if i == max_i && j == max_j => 1,
                            (_, _, 0) if surrounding_lights_count == 3 => 1,
                            (_, _, 0) if surrounding_lights_count != 3 => 0,
                            (_, _, 1) if matches!(surrounding_lights_count, 2 | 3) => 1,
                            (_, _, 1) if !matches!(surrounding_lights_count, 2 | 3) => 0,
                            _ => unreachable!(),
                        }
                    })
                    .collect()
            })
            .collect();
    }
    let on_count = lights
        .iter()
        .map(|row| row.iter().filter(|light| **light == 1).count())
        .sum::<usize>();
    println!("{}", on_count);
}
