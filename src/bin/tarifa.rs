use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut s = s.lines();

    let x = s.next().map(|l| l.parse::<usize>().unwrap()).unwrap();

    let v = s
        .skip(1)
        .fold(x, |acc, p| acc + x - p.parse::<usize>().unwrap());

    println!("{}", v);
}
