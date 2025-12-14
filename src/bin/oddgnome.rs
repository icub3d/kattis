use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    for m in s.lines().skip(1) {
        let mut vv = m
            .split_whitespace()
            .skip(1)
            .map(|v| v.parse::<usize>().unwrap());

        let mut cur = vv.next().unwrap();
        let mut n = 2;
        // && after let Some() new feature.
        while let Some(next) = vv.next()
            && next == cur + 1
        {
            n += 1;
            cur = next;
        }

        println!("{}", n);
    }
}
