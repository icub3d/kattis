use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    for m in s.lines().skip(1).map(|l| l.parse::<isize>().unwrap()) {
        if m % 2 == 0 {
            println!("{m} is even");
        } else {
            println!("{m} is odd");
        }
    }
}
