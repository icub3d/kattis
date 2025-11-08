use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut nn = s
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    nn.sort();
    let d1 = nn[1] - nn[0];
    let d2 = nn[2] - nn[1];

    match d1.cmp(&d2) {
        std::cmp::Ordering::Less => println!("{}", nn[1] + d1),
        std::cmp::Ordering::Greater => println!("{}", nn[0] + d2),
        std::cmp::Ordering::Equal => println!("{}", nn[2] + d2),
    }
}
