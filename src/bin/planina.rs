use std::io::{BufRead, Read, stdin};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let x = stdin().lock().lines().next().unwrap()?.parse::<i32>()?;
    // 2^x+1
    let n = (1 << x) + 1;
    println!("{:?}", n * n);
    Ok(())
}
// x  s  a
// 0  1  4
// 1  4  9 (2^1 + 1) ^ 2
// 2 16 25 (2^2 + 1) ^ 2
// 3 64 81 (2^3 + 1) ^ 2
// (2^x + 1) ^ 2
