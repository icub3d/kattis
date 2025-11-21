use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let n = s.trim().parse::<usize>().unwrap();

    for m in 1..=n {
        println!("{} Abracadabra", m);
    }
}
