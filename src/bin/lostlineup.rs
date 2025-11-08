use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut ss = s.lines();

    let n = ss.next().map(|v| v.parse::<u64>().unwrap()).unwrap();
    print!("1");
    if n == 1 {
        return;
    }

    let mut p = ss
        .next()
        .map(|r| {
            r.split_whitespace()
                .enumerate()
                .map(|(i, v)| (i + 2, v.parse::<i64>().unwrap()))
                .collect::<Vec<_>>()
        })
        .unwrap();
    p.sort_by_key(|(_, v)| *v);
    p.iter().for_each(|(i, _)| print!(" {}", i));
}
