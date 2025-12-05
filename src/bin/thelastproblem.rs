use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    println!("Thank you, {}, and farewell!", s.trim());
}
