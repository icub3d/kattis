use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();
    let (l, r) = s
        .trim()
        .split_once(' ')
        .map(|(l, r)| {
            (
                l.chars().rev().collect::<String>().parse::<u32>().unwrap(),
                r.chars().rev().collect::<String>().parse::<u32>().unwrap(),
            )
        })
        .unwrap();
    println!("{}", l.max(r));
}
