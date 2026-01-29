use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut ss = s.lines();

    while let Some(n) = ss.next() {
        if n == "0" {
            break;
        }

        let m = n
            .split_once(' ')
            .map(|(_, r)| r.parse::<usize>().unwrap())
            .unwrap();
        let taken = ss.next().unwrap().split_whitespace().collect::<Vec<_>>();
        let mut good = true;
        for _ in 0..m {
            let mut cat = ss.next().unwrap().split_whitespace();
            let _ = cat.next().unwrap();
            let req = cat.next().unwrap().parse::<usize>().unwrap();

            if req > cat.filter(|r| taken.contains(r)).count() {
                good = false;
            }
        }
        if good {
            println!("yes");
        } else {
            println!("no");
        }
    }
}
