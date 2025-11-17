use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let values = s.lines().nth(1).unwrap();

    let (n, t) = values
        .split_whitespace()
        .map(|i| i.parse::<f32>().unwrap())
        .filter(|i| *i >= 0.)
        .fold((0, 0.), |(n, t), i| (n + 1, t + i));

    println!("{}", t / n as f32);
}
