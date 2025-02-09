use std::io;

fn main() {
    let mut encoded_len: usize = 0;
    let mut string_len: usize = 0;
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
            for ch in line.chars() {
                match ch {
                    '\\' | '"' => {
                        encoded_len += 2;
                        string_len += 1;
                    }
                    _ => {
                        encoded_len += 1;
                        string_len += 1;
                    }
                }
            }
            // To account for the first and last double quotes
            encoded_len += 2;
        });
    println!("{}", encoded_len - string_len);
}
