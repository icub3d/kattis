use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();
    let (x, y) = s.trim().split_once(' ').unwrap();
    let (x, y) = (x.parse::<f32>().unwrap(), y.parse::<f32>().unwrap());

    // n = n*y + x
    // 0 = yn - n + x
    // n = x / (1-y)

    let d = 1. - y;
    let n = x / d;
    if d.abs() < 1e-6 {
        if x.abs() < 1e-6 {
            // 0/0
            println!("ALL GOOD");
        } else {
            // x/0
            println!("IMPOSSIBLE");
        }
    } else if n.fract().abs() < 1e-6 {
        println!("{}", n.round() as i32);
    } else {
        println!("{:.9}", n);
    }
}
