use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    for m in s.lines().skip(1) {
        let p = m
            .split_whitespace()
            .map(|p| p.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let mut cur = p[1];
        for (i, next) in p.iter().enumerate().skip(2) {
            if *next != cur + 1 {
                println!("{i}");
                break;
            }
            cur = *next;
        }
    }
}
