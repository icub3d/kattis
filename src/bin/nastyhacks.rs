use std::{
    cmp,
    io::{Read, stdin},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;
    let mut it = s.lines();

    let n = it.next().unwrap().parse::<usize>()?;

    for m in it {
        let p = m
            .split_whitespace()
            .map(|p| p.parse::<i32>())
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let (r, e, c) = (p[0], p[1], p[2]);

        match r.cmp(&(e - c)) {
            cmp::Ordering::Equal => println!("does not matter"),
            cmp::Ordering::Less => println!("advertise"),
            cmp::Ordering::Greater => println!("do not advertise"),
        }
    }
    Ok(())
}
