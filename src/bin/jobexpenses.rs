use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    for m in s.lines().skip(1) {
        let v = m
            .split_whitespace()
            .map(|v| v.parse::<isize>().unwrap())
            .filter(|v| *v < 0)
            .map(|v| v.abs())
            .sum::<isize>();
        println!("{v}");
    }
}
