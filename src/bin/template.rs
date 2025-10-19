use std::io::{Read, stdin};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;
    let mut it = s.lines();

    let x = it.next().unwrap().parse::<usize>()?;
    let t = it.next().unwrap();

    for m in it {}
    println!("{}", t);
    Ok(())
}
