use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut ll = s.lines().filter(|l| !l.trim().is_empty());

    let t = ll.next().unwrap().parse::<usize>().unwrap();

    for _ in 0..t {
        ll.next();
        let g = ll
            .next()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .max()
            .unwrap();
        let mg = ll
            .next()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .max()
            .unwrap();
        if g >= mg {
            println!("Godzilla");
        } else {
            println!("MechaGodzilla");
        }
    }
}
