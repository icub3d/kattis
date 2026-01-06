use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let tt = s
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let (mut d, mut t) = (1, 100);
    for i in 0..(tt.len() - 2) {
        let cur = tt[i].max(tt[i + 2]);
        if cur < t {
            t = cur;
            d = i + 1;
        }
    }
    println!("{d} {t}");
}
