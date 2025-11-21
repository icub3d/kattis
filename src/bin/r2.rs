use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let (l, r) = s
        .trim()
        .split_once(' ')
        .map(|(l, r)| (l.parse::<isize>().unwrap(), r.parse::<isize>().unwrap()))
        .unwrap();

    println!("{}", 2 * r - l);
}
