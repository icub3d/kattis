use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    for m in s.lines() {
        if m == "0 0 0 0" {
            break;
        }

        let nn = m
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let mut ticks = 80; // 2 turns
        ticks += (nn[0] - nn[1] + 40) % 40; // move to first.
        ticks += 40; // 1 turn;
        ticks += (nn[2] - nn[1] + 40) % 40; // move to second.
        ticks += (nn[2] - nn[3] + 40) % 40; // move to third.
        println!("{}", ticks * (360 / 40));
    }
}
