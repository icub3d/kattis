use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut lines = s.lines();

    let mut nn = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    nn.sort();

    let order = lines.next().unwrap().bytes().collect::<Vec<_>>();

    for c in order {
        print!("{} ", nn[(c - b'A') as usize]);
    }
}
