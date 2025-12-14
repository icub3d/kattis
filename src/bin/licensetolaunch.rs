use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut m = s.lines();
    m.next().unwrap();
    let days = m
        .next()
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let i = days
        .iter()
        .rev() // TECHNICALLY CORRECT ?
        .enumerate()
        .min_by_key(|(_, v)| *v)
        .map(|(i, _)| i)
        .unwrap();
    println!("{i}");
}
