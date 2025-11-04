use std::io::{Read, stdin};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;
    let (x, y) = s.trim().split_once(' ').unwrap();
    let (x, y) = (x.parse::<f32>()?, y.parse::<f32>()?);
    // n = n*y + x
    // n = x / (1 - y)
    let p = 1. - y;
    let t = x / p;
    if p.abs() < 1e-9 {
        // If 0/0 == all good, x/0 == impossible
        if x.abs() < 1e-9 {
            println!("ALL GOOD");
        } else {
            println!("IMPOSSIBLE")
        }
    } else if t.fract().abs() < 1e-9 {
        println!("{}", t.round() as i32);
    } else {
        println!("{:.9}", t);
    }
    Ok(())
}
