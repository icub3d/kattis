use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let (n, m) = s
        .trim()
        .split_once(' ')
        .map(|(n, m)| (n.parse::<usize>().unwrap(), m.parse::<usize>().unwrap()))
        .unwrap();

    if n < m {
        println!(
            "Dr. Chaz will have {} piece{} of chicken left over!",
            m - n,
            if m - n == 1 { "" } else { "s" }
        );
    } else {
        println!(
            "Dr. Chaz needs {} more piece{} of chicken!",
            n - m,
            if n - m == 1 { "" } else { "s" }
        );
    }
}
