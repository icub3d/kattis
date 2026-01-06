use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let s = s.trim();

    // Last score won game! :)
    println!("{}", s.chars().nth(s.len() - 2).unwrap());
}
