use std::io::{Read, stdin};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;
    let (l, r) = s.trim().split_once(' ').unwrap();
    let (l, r) = (l.parse::<i32>()?, r.parse::<i32>()?);

    if l <= r {
        println!(
            "Dr. Chaz will have {} piece{} of chicken left over!",
            r - l,
            if r - l == 1 { "" } else { "s" }
        );
    } else {
        println!(
            "Dr. Chaz needs {} more piece{} of chicken!",
            l - r,
            if l - r == 1 { "" } else { "s" }
        );
    }

    Ok(())
}
