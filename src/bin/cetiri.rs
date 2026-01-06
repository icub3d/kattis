use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut nn = s
        .split_whitespace()
        .map(|v| v.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    nn.sort();

    let d1 = nn[1] - nn[0];
    let d2 = nn[2] - nn[1];
    if d1 == d2 {
        println!("{}", nn[2] + d1);
    } else if d1 > d2 {
        println!("{}", nn[0] + d2);
    } else {
        println!("{}", nn[1] + d1);
    }
}
