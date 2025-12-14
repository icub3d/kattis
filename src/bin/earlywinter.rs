use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut lines = s.lines();
    let (_, d_m) = lines
        .next()
        .unwrap()
        .split_once(' ')
        .map(|(l, r)| (l, r.parse::<usize>().unwrap()))
        .unwrap();

    match lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse::<usize>().unwrap())
        .enumerate()
        .find(|(_, m)| *m <= d_m)
    {
        Some((i, _)) => println!("It hadn't snowed this early in {i} years!"),
        None => println!("It had never snowed this early!"),
    };
}
