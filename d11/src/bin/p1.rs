use std::io;

fn main() {
    let mut password = io::stdin()
        .lines()
        .next()
        .expect("Empty input")
        .expect("Stdin error");
    assert!(password.len() == 8, "Wrong password length");
    assert!(
        password.chars().all(|ch| ch.is_ascii_lowercase()),
        "Invalid characters"
    );
    let new_password = loop {
        increment(&mut password);
        if is_valid(&password) {
            break password;
        }
    };
    println!("{}", new_password);
}

fn is_valid(password: &str) -> bool {
    let condition_1 = password
        .as_bytes()
        .windows(3)
        .any(|window| window[1] == window[0] + 1 && window[2] == window[1] + 1);
    let condition_2 = password.chars().all(|ch| !matches!(ch, 'i' | 'o' | 'l'));
    let condition_3 = {
        let mut distinct_double_count = 0;
        let mut skip_flag = false;
        for window in password.as_bytes().windows(2) {
            if skip_flag {
                skip_flag = false;
            } else if window[0] == window[1] {
                distinct_double_count += 1;
                skip_flag = true;
            }
        }
        distinct_double_count == 2
    };
    condition_1 && condition_2 && condition_3
}

fn increment(password: &mut str) {
    // This is safe since we have already asserted
    // that the string only contains ascii chars.
    for ch in unsafe { password.as_bytes_mut().iter_mut().rev() } {
        *ch += 1;
        if *ch > b'z' {
            *ch = b'a';
        } else {
            break;
        }
    }
}
