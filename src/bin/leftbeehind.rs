use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    for m in s.lines() {
        if m == "0 0" {
            break;
        }

        let (s, x) = m
            .split_once(' ')
            .map(|(l, r)| (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()))
            .unwrap();

        if x + s == 13 {
            println!("Never speak again.")
        } else if x > s {
            println!("Left beehind.");
        } else if s > x {
            println!("To the convention.");
        } else if s == x {
            println!("Undecided.");
        }
    }
}
