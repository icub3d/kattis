use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();
    let s = s
        .split_whitespace()
        .map(|p| p.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let (x, y, n) = (s[0], s[1], s[2]);

    for i in 1..=n {
        if i % x == 0 && i % y == 0 {
            println!("FizzBuzz");
        } else if i % x == 0 {
            println!("Fizz");
        } else if i % y == 0 {
            println!("Buzz");
        } else {
            println!("{i}");
        }
    }
}
