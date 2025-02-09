use std::io;

enum Node {
    Integer(i64),
    String(String),
    Array(Vec<Node>),
    Map(Vec<Node>),
}

fn main() {
    let input = io::stdin()
        .lines()
        .next()
        .expect("Empty input")
        .expect("Stdin error");
    let root = match input.as_bytes()[0] {
        b'[' => Node::Array(parse_array(&input[1..input.len() - 1])),
        b'{' => Node::Map(parse_map(&input[1..input.len() - 1])),
        b'"' => Node::String(input[1..input.len() - 1].to_owned()),
        ch if ch.is_ascii_digit() || ch == b'-' => {
            Node::Integer(input.parse::<i64>().expect("Bad integer"))
        }
        _ => panic!("Bad character"),
    };
    println!("{}", sum(&root))
}

fn sum(node: &Node) -> i64 {
    match node {
        Node::Integer(n) => *n,
        Node::String(_) => 0,
        Node::Array(a) => a.iter().map(sum).sum(),
        Node::Map(m) => m.iter().map(sum).sum(),
    }
}

fn parse_map(mut input: &str) -> Vec<Node> {
    let mut children: Vec<Node> = Vec::new();
    while !input.is_empty() {
        let (child_str, rest) = pop_front_child_map(input);
        input = rest;
        let child = parse_node(child_str);
        if matches!(child, Node::String(ref s) if s == "red") {
            return Vec::new();
        }
        children.push(child);
    }
    children
}

fn parse_array(mut input: &str) -> Vec<Node> {
    let mut children: Vec<Node> = Vec::new();
    while !input.is_empty() {
        let (child_str, rest) = pop_front_child_array(input);
        input = rest;
        let child = parse_node(child_str);
        children.push(child);
    }
    children
}

fn parse_node(child_str: &str) -> Node {
    match child_str.as_bytes()[0] {
        b'[' => Node::Array(parse_array(&child_str[1..child_str.len() - 1])),
        b'{' => Node::Map(parse_map(&child_str[1..child_str.len() - 1])),
        b'"' => Node::String(child_str[1..child_str.len() - 1].to_owned()),
        ch if ch.is_ascii_digit() || ch == b'-' => {
            Node::Integer(child_str.parse::<i64>().expect("Bad integer"))
        }
        _ => panic!("Bad character"),
    }
}

fn pop_front_child_array(input: &str) -> (&str, &str) {
    pop_front_child(input)
}

fn pop_front_child_map(mut input: &str) -> (&str, &str) {
    let idx = input.find(':').expect("Malformed JSON object");
    (_, input) = input.split_at(idx + 1);
    pop_front_child(input)
}

fn pop_front_child(input: &str) -> (&str, &str) {
    let split_idx_opt = match input.as_bytes()[0] {
        b'[' => {
            let mut count = 1;
            let mut idx = None;
            for (i, ch) in input.chars().enumerate().skip(1) {
                match ch {
                    '[' => count += 1,
                    ']' => count -= 1,
                    _ => (),
                }
                if count == 0 {
                    idx = Some(i);
                    break;
                }
            }
            idx
        }
        b'{' => {
            let mut count = 1;
            let mut idx = None;
            for (i, ch) in input.chars().enumerate().skip(1) {
                match ch {
                    '{' => count += 1,
                    '}' => count -= 1,
                    _ => (),
                }
                if count == 0 {
                    idx = Some(i);
                    break;
                }
            }
            idx
        }
        b'"' => {
            let mut idx = None;
            for (i, ch) in input.chars().enumerate().skip(1) {
                if ch == '"' {
                    idx = Some(i);
                    break;
                }
            }
            idx
        }
        ch if ch.is_ascii_digit() || ch == b'-' => {
            let mut idx = Some(0);
            for (i, ch) in input.chars().enumerate().skip(1) {
                if ch.is_ascii_digit() {
                    idx = Some(i);
                } else {
                    break;
                }
            }
            idx
        }
        _ => panic!("Bad character"),
    };
    let split_idx = split_idx_opt.expect("Malformed JSON object");
    let (child_str, rest) = input.split_at(split_idx + 1);
    let rest = if !rest.is_empty() && rest.as_bytes()[0] == b',' {
        &rest[1..]
    } else {
        rest
    };
    (child_str, rest)
}
