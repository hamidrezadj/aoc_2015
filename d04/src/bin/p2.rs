use std::io;

fn main() {
    let input = io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .next()
        .expect("No input");
    for n in 1.. {
        let md5_input = input.clone() + &n.to_string();
        let digest = md5::compute(md5_input.as_bytes());
        let digest_string = format!("{:x}", digest);
        let first_five_string = &digest_string[0..6];
        if first_five_string == "000000" {
            println!("{}", n);
            return;
        }
    }
}
