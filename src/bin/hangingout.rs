use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut lines = s.lines();
    let limit = lines
        .next()
        .unwrap()
        .split_whitespace()
        .next()
        .map(|n| n.parse::<usize>().unwrap())
        .unwrap();

    let mut cur = 0;
    let mut denied = 0;

    for line in lines {
        let (w, n) = line
            .split_once(' ')
            .map(|(w, n)| (w, n.parse::<usize>().unwrap()))
            .unwrap();
        match w {
            "enter" => {
                if cur + n <= limit {
                    cur += n
                } else {
                    denied += 1
                }
            }
            _ => cur = cur.saturating_sub(n),
        }
    }
    println!("{denied}");
}
