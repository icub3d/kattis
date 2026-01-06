use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    for m in s.lines().skip(1) {
        let vv = m
            .split_whitespace()
            .map(|v| v.parse::<f64>().unwrap())
            .collect::<Vec<_>>();
        let (mut p, r, s) = (vv[0], vv[1], vv[2]);

        let mut year = 0;
        while p <= s {
            p = (p * r).floor();
            year += 1;
        }
        println!("{year}");
    }
}
