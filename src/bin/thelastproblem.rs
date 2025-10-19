use std::io::{BufRead, stdin};

fn main() {
    let input = stdin().lock().lines().next().unwrap().unwrap();
    let input = input.trim();
    println!("Thank you, {}, and farewell!", input);
}
