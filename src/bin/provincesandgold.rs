use std::io::{Read, stdin};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;
    let parts = s
        .trim()
        .split(' ')
        .map(|p| p.parse::<i32>())
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let (g, s, c) = (parts[0], parts[1], parts[2]);
    let bp = g * 3 + s * 2 + c;
    let treasure = if bp >= 6 {
        Some("Gold")
    } else if bp >= 3 {
        Some("Silver")
    } else {
        Some("Copper")
    };
    let vc = if bp >= 8 {
        Some("Province")
    } else if bp >= 5 {
        Some("Duchy")
    } else if bp >= 2 {
        Some("Estate")
    } else {
        None
    };
    match (treasure, vc) {
        (Some(t), Some(v)) => println!("{} or {}", v, t),
        (_, Some(v)) => println!("{}", v),
        (Some(v), _) => println!("{}", v),
        _ => (),
    };
    Ok(())
}
