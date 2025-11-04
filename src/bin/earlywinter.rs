use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut ss = s.lines();

    let (_, d_m) = ss
        .next()
        .unwrap()
        .split_once(' ')
        .map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
        .unwrap();

    for m in ss {
        let dd = m
            .split_whitespace()
            .map(|d| d.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        match dd.iter().position(|d| *d <= d_m) {
            Some(d) => println!("It hadn't snowed this early in {} years!", d),
            None => println!("It had never snowed this early!"),
        }
    }
}
