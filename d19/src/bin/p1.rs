use std::{collections::BTreeSet, io};

fn main() {
    let mut input_lines: Vec<String> = io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .collect();
    let molecule = input_lines.pop().expect("Empty input");
    let _empty_line = input_lines.pop();
    let replacements: Vec<(String, String)> = input_lines
        .into_iter()
        .map(|line| {
            let (lhs, rhs) = line
                .split_once(" => ")
                .expect("Invalid pattern: needs to have ' => ' in line");
            (lhs.to_owned(), rhs.to_owned())
        })
        .collect();
    assert!(!replacements.is_empty(), "No replacments provided");
    let mut new_molecules: BTreeSet<String> = BTreeSet::new();
    for (replacee, replacement) in replacements {
        let mut split_idx = 0;
        let (mut back_window, mut search_window) = molecule.split_at(0);
        while let Some(find_idx) = search_window.find(&replacee) {
            let new_molecule =
                back_window.to_owned() + &search_window.replacen(&replacee, &replacement, 1);
            new_molecules.insert(new_molecule);
            split_idx += find_idx + 1;
            (back_window, search_window) = molecule.split_at(split_idx);
        }
    }
    let count = new_molecules.len();
    println!("{}", count);
}
