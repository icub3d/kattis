use std::io::{Read, stdin};

fn score(d1: u32, d2: u32) -> u32 {
    match (d1.max(d2), d1.min(d2)) {
        (2, 1) => 1000,
        (h, l) if h == l => 600 + h * 10,
        (h, l) => 100 + h * 10 + l,
    }
}

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    for m in s.lines() {
        if m.trim() == "0 0 0 0" {
            break;
        }

        let pp = m
            .split_whitespace()
            .map(|p| p.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        match score(pp[0], pp[1]).cmp(&score(pp[2], pp[3])) {
            std::cmp::Ordering::Greater => println!("Player 1 wins."),
            std::cmp::Ordering::Less => println!("Player 2 wins."),
            _ => println!("Tie."),
        }
    }
}
