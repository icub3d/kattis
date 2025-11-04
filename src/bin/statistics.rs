use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    for (i, m) in s.lines().enumerate() {
        let n = m
            .split_whitespace()
            .skip(1)
            .map(|p| p.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let min = n.iter().min().unwrap();
        let max = n.iter().max().unwrap();
        println!("Case {}: {min} {max} {}", i + 1, max - min);
    }
}
