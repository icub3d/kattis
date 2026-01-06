use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut lines = s.lines();

    let n = lines.next().unwrap().parse::<usize>().unwrap();
    let nn = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut order = vec![0; n];
    order[0] = 1;

    for (i, d) in nn.iter().enumerate() {
        order[d + 1] = i + 2;
    }

    order.iter().for_each(|n| print!("{n} "));
    println!();
}
