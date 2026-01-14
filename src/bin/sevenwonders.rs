use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut counts = [0; 3];
    for c in s.trim().chars() {
        match c {
            'T' => counts[0] += 1,
            'C' => counts[1] += 1,
            _ => counts[2] += 1,
        }
    }

    let mut total = counts.iter().map(|&x| x * x).sum::<usize>();

    total += counts.iter().min().unwrap() * 7;

    println!("{total}");
}
