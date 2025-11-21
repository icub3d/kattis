use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let s = s.lines().collect::<Vec<_>>();
    let (_, c) = s[0].split_once(' ').unwrap();
    println!("{c}");
}
