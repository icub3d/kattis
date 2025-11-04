use std::io::{Read, stdin};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;
    let (l, r) = s.trim().split_once(' ').unwrap();
    let (l, r) = (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap());

    if l == 0 && r == 0 {
        println!("Not a moose");
    } else if l == r {
        println!("Even {}", l * 2);
    } else {
        println!("Odd {}", l.max(r) * 2);
    }
    Ok(())
}
