use std::{collections::BTreeMap, io};

enum InputGate {
    Buffer(String),
    Not(String),
    And(String, String),
    Or(String, String),
    LeftShift(String, usize),
    RightShift(String, usize),
}

fn main() {
    let circuit: BTreeMap<String, InputGate> = io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(parse_line)
        .collect();
    let mut memo: BTreeMap<String, u16> = BTreeMap::new();
    let a = evaluate("a", &circuit, &mut memo);
    println!("{}", a);
}

fn evaluate(
    node_id: &str,
    circuit: &BTreeMap<String, InputGate>,
    memo: &mut BTreeMap<String, u16>,
) -> u16 {
    if let Some(evaluation) = memo.get(node_id) {
        return *evaluation;
    }
    let node = circuit.get(node_id);
    let mut eval = |arg| evaluate(arg, circuit, memo);
    let evaluation = match node {
        Some(gate) => match gate {
            InputGate::Buffer(arg) => eval(arg),
            InputGate::Not(arg) => !eval(arg),
            InputGate::And(arg1, arg2) => eval(arg1) & eval(arg2),
            InputGate::Or(arg1, arg2) => eval(arg1) | eval(arg2),
            InputGate::LeftShift(arg, n) => eval(arg) << n,
            InputGate::RightShift(arg, n) => eval(arg) >> n,
        },
        None => node_id.parse::<u16>().unwrap_or_else(|_| {
            panic!(
                "Bad integer {}: Nodes that don't have a input gate are required to be u16.",
                node_id
            )
        }),
    };
    memo.insert(node_id.to_string(), evaluation);
    evaluation
}

fn parse_line(line: String) -> (String, InputGate) {
    let mut splits = line.split_whitespace().rev();
    let token = splits.next().expect("Invalid pattern: Empty line");
    if token.chars().any(|ch| !char::is_ascii_alphabetic(&ch)) {
        panic!("Invalid pattern: Node id contains a non 'alphabetic ascii' character");
    }
    let id = token.to_string();
    splits
        .next()
        .filter(|s| *s == "->")
        .expect("Invalid pattern: missing ->");
    let first = splits.next();
    let second = splits.next();
    let third = splits.next();
    let input = match (third, second, first) {
        (_, _, None) => panic!("Invalid pattern: No input"),
        (_, None, Some(t)) => InputGate::Buffer(t.to_string()),
        (None, Some("NOT"), Some(t)) => InputGate::Not(t.to_string()),
        (Some(t1), Some("AND"), Some(t2)) => InputGate::And(t1.to_string(), t2.to_string()),
        (Some(t1), Some("OR"), Some(t2)) => InputGate::Or(t1.to_string(), t2.to_string()),
        (Some(t1), Some("LSHIFT"), Some(t2)) => InputGate::LeftShift(
            t1.to_string(),
            t2.parse::<usize>().expect("Invaplid pattern: Bad integer"),
        ),
        (Some(t1), Some("RSHIFT"), Some(t2)) => InputGate::RightShift(
            t1.to_string(),
            t2.parse::<usize>().expect("Invaplid pattern: Bad integer"),
        ),
        _ => panic!("Invalid pattern: Should be in the form of 'lhs GATE rhs -> id'"),
    };
    (id, input)
}
