use std::io::{Read, stdin};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;
    let mut it = s.lines();

    let x = it.next().unwrap().parse::<i32>()?;
    let y = it.next().unwrap().parse::<i32>()?;

    match (x > 0, y > 0) {
        (true, true) => println!("1"),
        (true, false) => println!("4"),
        (false, true) => println!("2"),
        (false, false) => println!("3"),
    }
    Ok(())
}
