use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let n = s.trim().parse::<i32>().unwrap();

    // <= 3 == 1
    println!("{}", (n - 2).max(1));
}
