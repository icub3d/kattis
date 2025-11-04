use std::io::{Read, stdin};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;
    let s = s.trim();

    if s == "OCT 31" || s == "DEC 25" {
        println!("yup");
    } else {
        println!("nope");
    }

    Ok(())
}
