use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let (_, c) = s.lines().next().unwrap().split_once(' ').unwrap();

    println!("{c}");
}
