use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let pp = s
        .split_whitespace()
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let total = pp[0] * 3 + pp[1] * 2 + pp[2];

    let d = if total >= 8 {
        "Province"
    } else if total >= 5 {
        "Duchy"
    } else if total >= 2 {
        "Estate"
    } else {
        ""
    };

    let t = if total >= 6 {
        "Gold"
    } else if total >= 3 {
        "Silver"
    } else {
        "Copper"
    };

    if !d.is_empty() {
        println!("{} or {}", d, t);
    } else {
        println!("{}", t);
    }
}
