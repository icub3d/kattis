use std::io::{Read, stdin};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;
    let mut it = s.lines();

    let x = it.next().unwrap().parse::<usize>()?;

    for m in it {
        if m == "P=NP" {
            println!("skipped");
            continue;
        }

        let (l, r) = m.split_once('+').unwrap();
        println!("{}", l.parse::<i32>()? + r.parse::<i32>()?);
    }
    Ok(())
}
