use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let vv = s
        .split_whitespace()
        .map(|v| v.parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    let (p, a, b, c, d, n) = (vv[0], vv[1], vv[2], vv[3], vv[4], vv[5] as usize);

    let mut max_p = -1.0;
    let mut max_d = 0.0f64;

    for k in 1..=n {
        // PARENS LOLOLOLOL
        let pk = p * ((a * k as f64 + b).sin() + (c * k as f64 + d).cos() + 2.0);

        if pk > max_p {
            max_p = pk;
        } else if max_p - pk > max_d {
            max_d = max_p - pk;
        }
    }

    println!("{:.6}", max_d);
}
