use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    // I = C /  A
    // C = I / A
    let (a, i) = s
        .trim()
        .split_once(' ')
        .map(|(a, i)| (a.parse::<usize>().unwrap(), i.parse::<usize>().unwrap()))
        .unwrap();

    println!("{}", (i * a) - a + 1);
}
