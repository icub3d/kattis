use std::io::{Read, stdin};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input)?;
    let input = input.trim().parse::<i32>()?;

    for i in 1..=input {
        println!("{} Abracadabra", i);
    }
    Ok(())
}
