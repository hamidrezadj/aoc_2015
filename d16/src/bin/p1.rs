use std::{collections::BTreeMap, io};

fn main() {
    let target_properties: BTreeMap<String, u64> = [
        (String::from("children"), 3),
        (String::from("cats"), 7),
        (String::from("samoyeds"), 2),
        (String::from("pomeranians"), 3),
        (String::from("akitas"), 0),
        (String::from("vizslas"), 0),
        (String::from("goldfish"), 5),
        (String::from("trees"), 3),
        (String::from("cars"), 2),
        (String::from("perfumes"), 1),
    ]
    .into_iter()
    .collect();
    let sue_number = io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            let (_sue, properties) = line.split_once(": ").expect("Invalid pattern: no colons");
            properties
                .split(", ")
                .map(|p| p.split_once(": ").expect("Invalid pattern: no colons"))
                .map(|(p, n)| (p.trim().to_owned(), n.parse::<u64>().expect("Bad integer")))
                .collect::<BTreeMap<String, u64>>()
        })
        .enumerate()
        .filter(|(_i, properties)| {
            target_properties
                .iter()
                .all(|(target_property, target_value)| {
                    let property_value_option = properties.get(target_property);
                    property_value_option.is_none()
                        || target_value == property_value_option.unwrap()
                })
        })
        .map(|(i, _p)| i + 1)
        .next()
        .expect("No match found");
    println!("{}", sue_number);
}
