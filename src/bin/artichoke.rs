use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let pp = s
        .split_whitespace()
        .map(|p| p.parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    let (p, a, b, c, d, n) = (pp[0], pp[1], pp[2], pp[3], pp[4], pp[5] as usize);

    // Track current max and furthest drop. If we ever get a larger price, it will be the next max
    // because any values will differ by more with it. Then we just track the maximum drop of all
    // the drops.
    let mut it = (1..=n).map(|k| {
        let k = k as f64;
        p * ((a * k + b).sin() + (c * k + d).cos() + 2.)
    });
    let first = it.next().unwrap();
    let (_, drop) = it.fold((first, 0f64), |(max, drop), v| {
        (max.max(v), drop.max(max - v))
    });
    println!("{:.6}", drop);
}
