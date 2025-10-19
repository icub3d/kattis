use std::io::{BufRead, stdin};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    for line in stdin().lock().lines() {
        let line = line?;
        let (l, r) = line.split_once(' ').unwrap();
        let (l, r) = (l.parse::<i64>()?, r.parse::<i64>()?);
        println!("{}", (l - r).abs());
    }

    Ok(())
}
