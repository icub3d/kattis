use std::{
    cmp::Ordering,
    io::{Read, stdin},
};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    for m in s.lines().skip(1) {
        let pp = m
            .split_whitespace()
            .map(|v| v.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let (r, e, c) = (pp[0], pp[1], pp[2]);

        println!(
            "{}",
            match r.cmp(&(e - c)) {
                Ordering::Greater => "do not advertise",
                Ordering::Less => "advertise",
                _ => "does not matter",
            }
        );
    }
}
