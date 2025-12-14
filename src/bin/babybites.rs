use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut lines = s.lines();
    lines.next();

    if lines
        .next()
        .unwrap()
        .split_whitespace()
        .enumerate()
        .all(|(i, n)| n == "mumble" || i + 1 == n.parse::<usize>().unwrap())
    {
        println!("makes sense");
    } else {
        println!("something is fishy");
    }
}
