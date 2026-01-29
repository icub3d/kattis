use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut ss = s.lines();

    let mut ff = ss
        .next()
        .unwrap()
        .split_whitespace()
        .map(|f| f.parse::<usize>().unwrap());

    let (n, b, _, _) = (
        ff.next().unwrap(),
        ff.next().unwrap(),
        ff.next().unwrap(),
        ff.next().unwrap(),
    );

    let mut min = usize::MAX;
    while let Some(p) = ss.next() {
        let p = p.parse::<usize>().unwrap();
        let aa = ss
            .next()
            .unwrap()
            .split_whitespace()
            .map(|a| a.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        if aa.iter().all(|&a| a < n) {
            continue;
        }

        if n * p > b {
            continue;
        }

        min = min.min(n * p);
    }

    if min < b {
        println!("{min}");
    } else {
        println!("stay home");
    }
}
