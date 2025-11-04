use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    for m in s.lines().skip(1) {
        let p = m
            .split_whitespace()
            .map(|p| p.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let min = p.iter().min().unwrap();
        println!("{}", p.iter().position(|p| p == min).unwrap());
    }
}
