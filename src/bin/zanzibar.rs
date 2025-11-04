use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    for m in s.lines().skip(1) {
        let pp = m
            .split_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut c = 1;
        let mut delta = 0;
        for p in pp {
            if p == 0 {
                break;
            }
            if p <= c * 2 {
                c = p;
            } else {
                delta += p - c * 2;
                c = p;
            }
        }
        println!("{delta}");
    }
}
