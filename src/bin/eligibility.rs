use std::io::{Read, stdin};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;
    let mut it = s.lines();

    let x = it.next().unwrap().parse::<usize>()?;

    for m in it {
        let parts = m.split_whitespace().collect::<Vec<_>>();
        let (name, start, dob, courses) = (parts[0], parts[1], parts[2], parts[3].parse::<i32>()?);
        let start = start
            .split('/')
            .map(|d| d.parse::<i32>())
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let dob = dob
            .split('/')
            .map(|d| d.parse::<i32>())
            .collect::<std::result::Result<Vec<_>, _>>()?;

        if start[0] >= 2010 || dob[0] >= 1991 {
            println!("{} eligible", name);
        } else if courses >= 41 {
            println!("{} ineligible", name);
        } else {
            println!("{} coach petitions", name);
        }
    }
    Ok(())
}
