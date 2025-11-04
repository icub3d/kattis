use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    for m in s.lines().skip(1) {
        println!(
            "{}",
            m.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .filter(|v| *v < 0)
                .map(|v| v.abs())
                .sum::<i32>()
        );
    }
}
