use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut lines = s.lines();

    while let Some(line) = lines.next() {
        let n = match line.parse::<usize>() {
            Ok(n) => n,
            Err(_) => break,
        };

        let dist = lines
            .by_ref()
            .take(n)
            .scan(0, |prev, line| {
                let (speed, duration) = line
                    .split_once(' ')
                    .map(|(l, r)| (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()))
                    .unwrap();
                let cur = duration - *prev;
                *prev = duration;
                Some(speed * cur)
            })
            .sum::<usize>();

        println!("{} miles", dist);
    }
}
