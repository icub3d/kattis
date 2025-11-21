use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let n = s.trim().parse::<u32>().unwrap();

    // 0 4     2^2
    // 1 9     3^2 (2^1+1)^2
    // 2 25    5^2 (2^2+1)^2
    // 5 1089  33^2 (2^5+1)^2!

    println!("{}", (2_u64.pow(n) + 1).pow(2));
}
