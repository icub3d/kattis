use std::io::{Read, stdin};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;
    let mut it = s.lines();

    it.next().unwrap().parse::<usize>()?;

    for m in it {
        let m = m.parse::<i32>()?;
        println!("{} is {}", m, if m % 2 == 0 { "even" } else { "odd" });
    }
    Ok(())
}
