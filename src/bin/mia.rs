use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    for m in s.lines() {
        if m == "0 0 0 0" {
            break;
        }
        let vv = m
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let p1 = vv[0].max(vv[1]) * 10 + vv[1].min(vv[0]);
        let p2 = vv[2].max(vv[3]) * 10 + vv[3].min(vv[2]);

        if p1 == p2 {
            println!("Tie.");
        } else if p1 == 21 {
            println!("Player 1 wins.");
        } else if p2 == 21 {
            println!("Player 2 wins.");
        } else if p1 % 11 == 0 && p2 % 11 != 0 {
            println!("Player 1 wins.");
        } else if p2 % 11 == 0 && p1 % 11 != 0 {
            println!("Player 2 wins.");
        } else if p1 > p2 {
            println!("Player 1 wins.");
        } else {
            println!("Player 2 wins.");
        }
    }
}
