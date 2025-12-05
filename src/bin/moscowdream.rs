use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let pp = s
        .split_whitespace()
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let (a, b, c, n) = (pp[0], pp[1], pp[2], pp[3]);

    if a > 0 && b > 0 && c > 0 && a + b + c >= n && n >= 3 {
        println!("YES");
    } else {
        println!("NO");
    }
}
