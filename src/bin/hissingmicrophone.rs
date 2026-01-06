use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    if s.contains("ss") {
        println!("hiss");
    } else {
        println!("no hiss");
    }
}
