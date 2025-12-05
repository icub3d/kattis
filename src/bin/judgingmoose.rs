use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let (l, r) = s
        .trim()
        .split_once(' ')
        .map(|(l, r)| (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()))
        .unwrap();

    if l == 0 && r == 0 {
        println!("Not a moose");
    } else {
        println!("{} {}", if l == r { "Even" } else { "Odd" }, l.max(r) * 2);
    }
}
