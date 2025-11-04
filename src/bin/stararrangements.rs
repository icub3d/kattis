use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let s = s.trim().parse::<i32>().unwrap();

    println!("{s}:");
    for n in 2..s {
        if s % (n + n - 1) == 0 || s % (n + n - 1) == n {
            println!("{n},{}", n - 1);
        }
        if s % n == 0 {
            println!("{n},{n}");
        }
    }
}
