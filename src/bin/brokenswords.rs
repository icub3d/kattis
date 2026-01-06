use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let (v, h) = s.lines().skip(1).fold((0, 0), |(mut v, mut h), l| {
        let n = isize::from_str_radix(l, 2).unwrap();
        if n & 8 == 0 {
            v += 1;
        }
        if n & 4 == 0 {
            v += 1;
        }
        if n & 2 == 0 {
            h += 1;
        }
        if n & 1 == 0 {
            h += 1;
        }
        (v, h)
    });

    let s = (v / 2).min(h / 2);

    let rv = v - (s * 2);
    let rh = h - (s * 2);
    println!("{s} {rv} {rh}");
}
