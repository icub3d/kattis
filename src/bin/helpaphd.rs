use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    for m in s.lines().skip(1) {
        if m == "P=NP" {
            println!("skipped");
            continue;
        }

        let (l, r) = m
            .split_once('+')
            .map(|(l, r)| (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()))
            .unwrap();

        println!("{}", l + r);
    }
}
