use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let s = s.trim().parse::<usize>().unwrap();

    println!("{s}:");
    (2..=(s / 2 + 1))
        .flat_map(|n| [(n, n - 1), (n, n)])
        .filter(|(x, y)| s % (x + y) == 0 || s % (x + y) == *x)
        .for_each(|(x, y)| println!("{x},{y}"));
}
