use std::io::{Read, stdin};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;
    let mut it = s.lines();

    let x = it.next().unwrap().parse::<usize>()?;
    let _ = it.next().unwrap();

    let mut n = 0;
    for m in it {
        n += x;
        let m = m.parse::<usize>()?;
        n = n.saturating_sub(m);
    }
    println!("{:?}", n + x);
    Ok(())
}
