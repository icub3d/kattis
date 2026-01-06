use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let names = s.lines().skip(1).collect::<Vec<_>>();

    let mut increasing = names.clone();
    increasing.sort();

    let mut decreasing = increasing.clone();
    decreasing.sort_by(|a, b| b.cmp(a));

    if names == increasing {
        println!("INCREASING");
    } else if names == decreasing {
        println!("DECREASING");
    } else {
        println!("NEITHER");
    }
}
