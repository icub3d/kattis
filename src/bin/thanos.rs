use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    for m in s.lines().skip(1) {
        let pp = m
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let (mut p, r, f) = (pp[0], pp[1], pp[2]);

        let mut n: usize = 0;
        while p <= f {
            p *= r;
            n += 1;
        }

        println!("{n}");
    }
}
