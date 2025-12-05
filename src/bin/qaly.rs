use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let q = s
        .lines()
        .skip(1)
        .map(|l| {
            l.split_once(' ')
                .map(|(l, r)| (l.parse::<f64>().unwrap(), r.parse::<f64>().unwrap()))
                .unwrap()
        })
        .map(|(l, r)| l * r)
        .sum::<f64>();

    println!("{:.3}", q);
}
