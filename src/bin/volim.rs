use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut ss = s.lines();
    let mut cur = ss.next().unwrap().parse::<isize>().unwrap();
    ss.next();

    let mut time = 210;

    for l in ss {
        let (t, r) = l
            .split_once(' ')
            .map(|(t, r)| (t.parse::<isize>().unwrap(), r))
            .unwrap();
        time -= t;
        if time <= 0 {
            break;
        }

        // 1's counting
        if r == "T" {
            cur = cur % 8 + 1;
        }
    }

    println!("{cur}");
}
