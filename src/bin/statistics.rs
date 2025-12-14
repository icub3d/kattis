use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    for (i, m) in s.lines().enumerate() {
        let vv = m
            .split_whitespace()
            .skip(1)
            .map(|v| v.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let min = vv.iter().min().unwrap();
        let max = vv.iter().max().unwrap();
        println!("Case {}: {} {} {}", i + 1, min, max, max - min);
    }
}
