use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut ss = s.split_whitespace().map(|v| v.parse::<isize>().unwrap());

    let (x0, y0, x1, y1, t) = (
        ss.next().unwrap(),
        ss.next().unwrap(),
        ss.next().unwrap(),
        ss.next().unwrap(),
        ss.next().unwrap(),
    );

    let dist = (x0 - x1).abs() + (y0 - y1).abs();

    // u-turn divisible  by 2
    if t >= dist && (t - dist) % 2 == 0 {
        println!("Y");
    } else {
        println!("N");
    }
}
