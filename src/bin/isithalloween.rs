use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let (m, d) = s.trim().split_once(' ').unwrap();

    if (m == "DEC" && d == "25") || (m == "OCT" && d == "31") {
        println!("yup");
    } else {
        println!("nope");
    }
}
