use std::io;

fn main() {
    let answer = io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .filter(|line| {
            line.chars()
                .filter(|ch| matches!(ch, 'a' | 'e' | 'i' | 'o' | 'u'))
                .count()
                >= 3
        })
        .filter(|line| {
            let mut last_ch = line.chars().next().expect("Empty line");
            for ch in line.chars().skip(1) {
                if ch == last_ch {
                    return true;
                }
                last_ch = ch;
            }
            false
        })
        .filter(|line| !line.contains("ab"))
        .filter(|line| !line.contains("cd"))
        .filter(|line| !line.contains("pq"))
        .filter(|line| !line.contains("xy"))
        .count();
    println!("{}", answer)
}
