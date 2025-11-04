use std::io::{Read, stdin};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;
    let mut it = s.lines();

    for m in it {
        if m == "0 0" {
            break;
        }

        let (l, r) = m.split_once(' ').unwrap();
        let (sweet, sour) = (l.parse::<i32>()?, r.parse::<i32>()?);
        if sweet + sour == 13 {
            println!("Never speak again.");
        } else if sour > sweet {
            println!("Left beehind.");
        } else if sweet > sour {
            println!("To the convention.");
        } else if sweet == sour {
            println!("Undecided.");
        }
    }
    Ok(())
}
