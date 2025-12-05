use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let n = s.trim().parse::<u32>().unwrap();

    println!("{}", (2i64.pow(n) + 1).pow(2))
}
// 0  4   (2^0 + 1)^2
// 1  9   (2^1 + 1)^2
// 2  25  (2^2 + 1)^2
// ..
// 5  1089 (2^5 + 1)^2
