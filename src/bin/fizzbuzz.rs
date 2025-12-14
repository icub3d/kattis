use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut vv = s.split_whitespace().map(|v| v.parse::<usize>().unwrap());

    let (x, y, n) = (vv.next().unwrap(), vv.next().unwrap(), vv.next().unwrap());

    for i in 1..=n {
        if i % x == 0 && i % y == 0 {
            println!("FizzBuzz");
        } else if i % x == 0 {
            println!("Fizz");
        } else if i % y == 0 {
            println!("Buzz");
        } else {
            println!("{i}");
        }
    }
}
