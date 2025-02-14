use std::{io, ops::Not};

type Element = (u8, u8);
type Molecule = Vec<Element>;

fn main() {
    let target_molecule = io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .last()
        .map(|m| parse_molecule(&m))
        .expect("Empty input");
    let open_parentheses = std::env::args()
        .nth(1)
        .map(|s| parse_element(&s))
        .unwrap_or_else(|| parse_element("Rn"));
    let comma = std::env::args()
        .nth(2)
        .map(|s| parse_element(&s))
        .unwrap_or_else(|| parse_element("Y"));
    let close_parentheses = std::env::args()
        .nth(3)
        .map(|s| parse_element(&s))
        .unwrap_or_else(|| parse_element("Ar"));
    let (elements, parentheses, commas) = target_molecule.into_iter().fold(
        (0, 0, 0),
        |(mut elements, mut parentheses, mut commas), element| {
            match element {
                e if e == open_parentheses || e == close_parentheses => parentheses += 1,
                e if e == comma => commas += 1,
                _ => (),
            }
            elements += 1;
            (elements, parentheses, commas)
        },
    );
    let output = elements - 1 - parentheses - 2 * commas;
    println!("{}", output);
}

fn parse_molecule(string: &str) -> Molecule {
    let mut bytes = string.bytes().collect::<Vec<u8>>();
    bytes.push(b'\0');
    bytes
        .windows(2)
        .filter(|window| window[0].is_ascii_uppercase() || window[0] == b'e')
        .map(|window| {
            if window[1].is_ascii_lowercase() {
                (window[0], window[1])
            } else {
                (window[0], b'\0')
            }
        })
        .collect::<Molecule>()
}

fn parse_element(string: &str) -> Element {
    assert!(string.is_empty().not(), "Empty element string");
    assert!(
        string.len() <= 2,
        "Element string has more than 2 character"
    );
    let bytes = string.bytes().collect::<Vec<u8>>();
    assert!(
        bytes.first().unwrap().is_ascii_uppercase(),
        "Element isn't capitalized"
    );
    assert!(bytes.get(1).is_none_or(|ch| ch.is_ascii_lowercase()));
    (
        bytes.first().copied().unwrap(),
        bytes.get(1).copied().unwrap_or(b'\0'),
    )
}
