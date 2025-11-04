use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut s = s.lines();

    loop {
        let n = s.next().map(|v| v.parse::<i32>().unwrap()).unwrap();
        if n == -1 {
            break;
        }

        let d = s
            .by_ref()
            .take(n as usize)
            .map(|l| {
                l.split_once(' ')
                    .map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
                    .unwrap()
            })
            .fold((0_i32, 0_i32), |(miles, prev), (speed, duration)| {
                (miles + (speed * (duration - prev)), duration)
            })
            .0;
        println!("{d} miles");
    }
}
