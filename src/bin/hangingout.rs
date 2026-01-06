use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut l = s.lines();

    let max = l
        .next()
        .unwrap()
        .split_once(' ')
        .unwrap()
        .0
        .parse::<usize>()
        .unwrap();

    let mut cur = 0;
    let mut denied = 0;

    for m in l {
        let (e, v) = m.split_once(' ').unwrap();
        let v = v.parse::<usize>().unwrap();
        match e {
            "enter" => {
                if cur + v > max {
                    denied += 1;
                } else {
                    cur += v;
                }
            }
            _ => {
                cur -= v;
            }
        }
    }
    println!("{denied}");
}
