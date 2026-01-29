use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut ss = s.split_whitespace().map(|v| v.parse::<isize>().unwrap());
    let n = ss.next().unwrap();
    let r = ss.next().unwrap();
    let k = ss.next().unwrap();

    let to_desk = k + (r - k).abs();

    // If we've walked n, we are done.
    // Otherwise we need to walk more steps but can only do it in pairs of 2.
    let total = match to_desk < n {
        true => to_desk + ((n - to_desk + 1) / 2) * 2 + r,
        false => to_desk + r,
    };

    println!("{total}");
}
