use std::io;

fn main() {
    let answer = io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .filter(|line| {
            if line.len() < 4 {
                return false;
            }
            for boundary in 2..=line.len() - 2 {
                let (first, second) = line.split_at(boundary);
                let ref_couple = &first[first.len() - 2..first.len()];
                if second.contains(ref_couple) {
                    return true;
                }
            }
            false
        })
        .filter(|line| {
            let second_to_last_ch_opt = line.chars().next();
            let mut second_to_last_ch = match second_to_last_ch_opt {
                Some(c) => c,
                None => return false,
            };
            let last_ch_opt = line.chars().nth(1);
            let mut last_ch = match last_ch_opt {
                Some(c) => c,
                None => return false,
            };
            for ch in line.chars().skip(2) {
                if ch == second_to_last_ch {
                    return true;
                }
                second_to_last_ch = last_ch;
                last_ch = ch;
            }
            false
        })
        .count();
    println!("{}", answer)
}
