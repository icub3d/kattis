use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut ss = s.lines();
    let have = ss
        .next()
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    let want = ss
        .next()
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse::<f64>().unwrap())
        .collect::<Vec<_>>();

    let factor = have
        .iter()
        .zip(want.iter())
        .map(|(h, w)| h / w)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    println!(
        "{}",
        have.iter()
            .zip(want.iter())
            .map(|(h, w)| format!("{:.6}", h - w * factor))
            .collect::<Vec<_>>()
            .join(" ")
    );
}
