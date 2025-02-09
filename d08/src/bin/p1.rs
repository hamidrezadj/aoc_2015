use std::io;

fn main() {
    let mut string_len: usize = 0;
    let mut in_memory_len: usize = 0;
    io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .for_each(|line| {
            assert!(line.len() >= 2);
            assert_eq!(&line[0..1], "\"");
            assert_eq!(&line[line.len() - 1..line.len()], "\"");
            assert!(
                (&line[line.len() - 2..line.len() - 1] != "\\")
                    || (line.len() >= 4 && &line[line.len() - 3..line.len() - 2] == "\\")
            );
            let mut chars = line.chars();
            while let Some(ch) = chars.next() {
                if ch != '\\' {
                    string_len += 1;
                    in_memory_len += 1;
                    continue;
                }
                let mut next_ch = || chars.next().unwrap();
                match next_ch() {
                    '\\' | '"' => {
                        string_len += 2;
                        in_memory_len += 1;
                    }
                    'x' => {
                        match (next_ch(), next_ch()) {
                            ('0'..='9' | 'a'..='f', '0'..='9' | 'a'..='f') => (),
                            _ => panic!("Invalid pattern: Non hex code after hex escape sequence"),
                        }
                        string_len += 4;
                        in_memory_len += 1;
                    }
                    _ => {
                        panic!("Invalid pattern: Can only escape \" or \\ or hex code (eg; \\xf8).")
                    }
                }
            }
            // To account for the first and last double quotes
            in_memory_len -= 2;
        });
    println!("{}", string_len - in_memory_len);
}
