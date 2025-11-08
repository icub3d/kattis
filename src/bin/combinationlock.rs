use std::io::{Read, stdin};

const DEG: i32 = 360 / 40;

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    for m in s.lines() {
        if m == "0 0 0 0" {
            break;
        }
        let nn = m
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let (start, first, second, third) = (nn[0], nn[1], nn[2], nn[3]);

        let total = cw(start, first) + ccw(first, second) + cw(second, third) + 1080;
        println!("{total}");
    }
}

fn ccw(a: i32, b: i32) -> i32 {
    (b - a).rem_euclid(40) * DEG
}

fn cw(a: i32, b: i32) -> i32 {
    (a - b).rem_euclid(40) * DEG
}
