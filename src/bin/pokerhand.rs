use std::{
    collections::HashMap,
    io::{Read, stdin},
};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut hand: HashMap<char, usize> = HashMap::new();
    s.split_whitespace()
        .map(|v| v.chars().next().unwrap())
        .for_each(|c| *hand.entry(c).or_default() += 1);

    println!("{}", hand.values().max().unwrap());
}
