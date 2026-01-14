use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();
    // $M$ , $P$ , $L$ , $E$ , $R$ , $S$ , $N$

    for m in s.lines() {
        let mut pp = m.split_whitespace().map(|l| l.parse::<usize>().unwrap());
        let (mut m, mut p, mut l, e, r, s, n) = (
            pp.next().unwrap(),
            pp.next().unwrap(),
            pp.next().unwrap(),
            pp.next().unwrap(),
            pp.next().unwrap(),
            pp.next().unwrap(),
            pp.next().unwrap(),
        );
        for _ in 0..n {
            let nl = m * e;
            let np = l / r;
            let na = p / s;
            l = nl;
            p = np;
            m = na;
        }
        println!("{}", m);
    }
}
