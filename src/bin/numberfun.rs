use std::io::{Read, stdin};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;

    for m in s.lines().skip(1) {
        let p = m
            .split_whitespace()
            .map(|p| p.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let (a, b, c) = (p[0], p[1], p[2]);
        if a + b == c
            || a - b == c
            || a * b == c
            || b - a == c
            || (b / a == c && b % a == 0)
            || (a / b == c && a % b == 0)
        {
            println!("Possible");
        } else {
            println!("Impossible");
        }
    }
    Ok(())
}
