use std::{
    cmp::Ordering,
    io::{Read, stdin},
};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut s = s.lines();
    let x = s.next().map(|l| l.parse::<isize>().unwrap()).unwrap();
    let y = s.next().map(|l| l.parse::<isize>().unwrap()).unwrap();

    match (x.cmp(&0), y.cmp(&0)) {
        (Ordering::Greater, Ordering::Greater) => println!("1"),
        (Ordering::Greater, Ordering::Less) => println!("4"),
        (Ordering::Less, Ordering::Less) => println!("3"),
        _ => println!("2"),
    }
}
