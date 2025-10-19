use std::io::{BufRead, stdin};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let qaly = stdin()
        .lock()
        .lines()
        .skip(1)
        .try_fold(0., |acc, l| -> Result<f32> {
            let l = l?;
            let (l, r) = l.split_once(' ').unwrap();
            let (l, r) = (l.parse::<f32>()?, r.parse::<f32>()?);
            Ok(acc + l * r)
        })?;

    println!("{:.3}", qaly);
    Ok(())
}
