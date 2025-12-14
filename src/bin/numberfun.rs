use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    for m in s.lines().skip(1) {
        let parts = m.split_whitespace().collect::<Vec<_>>();
        let (a, b, c) = (
            parts[0].parse::<usize>().unwrap(),
            parts[1].parse::<usize>().unwrap(),
            parts[2].parse::<usize>().unwrap(),
        );

        if a + b == c
            || a - b == c
            || b - a == c
            || a * b == c
            || (a / b == c && a % b == 0)
            || (b / a == c && b % a == 0)
        {
            println!("Possible");
        } else {
            println!("Impossible");
        }
    }
}
