use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let (r1, s) = s
        .trim()
        .split_once(' ')
        .map(|(l, r)| (l.parse::<isize>().unwrap(), r.parse::<isize>().unwrap()))
        .unwrap();

    println!("{}", 2 * s - r1);
}
