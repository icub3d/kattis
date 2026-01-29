use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut lines = s.lines();

    let mut pp = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<f64>().unwrap());

    let cap = pp.next().unwrap();
    let leak = pp.next().unwrap();
    let dist = pp.next().unwrap();
    let fuel = cap / 2.0;

    let speeds = lines
        .map(|l| {
            l.split_once(' ')
                .map(|(l, r)| (l.parse::<f64>().unwrap(), r.parse::<f64>().unwrap()))
                .unwrap()
        })
        .collect::<Vec<_>>();

    // Just find biggest speed we can go to get to distance.
    for (speed, mpg) in speeds.iter().rev() {
        let time = dist / speed;
        let consumed = dist / mpg;
        let leaked = leak * time;

        let needed = consumed + leaked;

        if needed <= fuel {
            println!("YES {}", *speed as isize);
            return;
        }
    }
    println!("NO");
}
