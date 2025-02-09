use std::io;

struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

fn main() {
    let input: Vec<Ingredient> = io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            let mut splits = line.split_whitespace();
            let capacity = splits
                .nth(2)
                .expect("Not enough words in line.")
                .split_terminator(',')
                .next()
                .unwrap()
                .parse::<i64>()
                .expect("Bad integer for capacity");
            let durability = splits
                .nth(1)
                .expect("Not enough words in line.")
                .split_terminator(',')
                .next()
                .unwrap()
                .parse::<i64>()
                .expect("Bad integer for durability");
            let flavor = splits
                .nth(1)
                .expect("Not enough words in line.")
                .split_terminator(',')
                .next()
                .unwrap()
                .parse::<i64>()
                .expect("Bad integer for flavor");
            let texture = splits
                .nth(1)
                .expect("Not enough words in line.")
                .split_terminator(',')
                .next()
                .unwrap()
                .parse::<i64>()
                .expect("Bad integer for texture");
            let calories = splits
                .nth(1)
                .expect("Not enough words in line.")
                .parse::<i64>()
                .expect("Bad integer for calories");
            Ingredient {
                capacity,
                durability,
                flavor,
                texture,
                calories,
            }
        })
        .collect();
    assert_eq!(
        input.len(),
        4,
        "This program is written for input of 4 lines."
    );
    let mut max_score = None;
    for x in 0..=100 {
        for y in 0..=100 {
            for z in 0..=100 {
                if x + y + z > 100 {
                    continue;
                }
                let w = 100 - (x + y + z);
                let calories = input[0].calories * x
                    + input[1].calories * y
                    + input[2].calories * z
                    + input[3].calories * w;
                if calories != 500 {
                    continue;
                }
                let score = (input[0].capacity * x
                    + input[1].capacity * y
                    + input[2].capacity * z
                    + input[3].capacity * w)
                    .max(0)
                    * (input[0].durability * x
                        + input[1].durability * y
                        + input[2].durability * z
                        + input[3].durability * w)
                        .max(0)
                    * (input[0].flavor * x
                        + input[1].flavor * y
                        + input[2].flavor * z
                        + input[3].flavor * w)
                        .max(0)
                    * (input[0].texture * x
                        + input[1].texture * y
                        + input[2].texture * z
                        + input[3].texture * w)
                        .max(0);
                if max_score.is_none() || max_score.unwrap() < score {
                    max_score = Some(score);
                }
            }
        }
    }
    println!("{}", max_score.unwrap());
}
