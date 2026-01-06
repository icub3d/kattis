use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let (l, r): (String, String) = s
        .trim()
        .split_once(' ')
        .map(|(l, r)| (l.chars().rev().collect(), r.chars().rev().collect()))
        .unwrap();

    let (l, r) = (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap());

    println!("{}", l.max(r));
}
