use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let b = s
        .split_whitespace()
        .skip(1)
        .map(|b| b.parse::<isize>().unwrap())
        .filter(|b| *b >= 0)
        .collect::<Vec<_>>();

    println!("{}", b.iter().sum::<isize>() as f64 / b.len() as f64);
}
