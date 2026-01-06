use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    for m in s.lines().skip(1) {
        let imports = m
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .filter(|&v| v != 0)
            .scan(None, |prev, cur| {
                let import = match *prev {
                    Some(p) => cur.saturating_sub(2 * p),
                    None => 0,
                };
                *prev = Some(cur);
                Some(import)
            })
            .sum::<usize>();
        println!("{imports}");
    }
}
