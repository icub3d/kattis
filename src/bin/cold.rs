use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    for m in s.lines().skip(1) {
        println!(
            "{}",
            m.split_whitespace()
                .map(|p| p.parse::<i32>().unwrap())
                .filter(|&t| t < 0)
                .count()
        );
    }
}
