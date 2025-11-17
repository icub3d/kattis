use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let days = s.lines().nth(1).unwrap();

    let days = days
        .split_whitespace()
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let (day, min) = days
        .windows(3)
        .zip(1..)
        .map(|(w, i)| (i, w[0].max(w[2])))
        .min_by_key(|(_, w)| *w)
        .unwrap();

    println!("{day} {min}");
}
