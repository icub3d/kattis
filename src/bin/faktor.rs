use std::io::{BufRead, stdin};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = stdin().lock().lines().next().unwrap()?;
    let (articles, impact) = input.split_once(' ').unwrap();
    let (articles, impact) = (articles.parse::<i32>()?, impact.parse::<i32>()?);
    // One more citation than the previous impact score would "round up"
    println!("{}", articles * (impact - 1) + 1);
    Ok(())
}
