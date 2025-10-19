use std::io::{BufRead, Result, stdin};

fn main() -> Result<()> {
    let input = stdin().lock().lines().collect::<Result<Vec<String>>>()?;
    // s= (r1 + r2) / 2
    // 2s = r1 + r2
    // r2 = 2s - r1
    let (r1, s) = input[0]
        .trim()
        .split_once(' ')
        .map(|(a, b)| (a.parse::<isize>().unwrap(), b.parse::<isize>().unwrap()))
        .unwrap();

    println!("{}", 2 * s - r1);

    Ok(())
}
