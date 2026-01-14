use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    // Both operations are opposites, just count.
    let (b, w) = s.trim().chars().fold((0, 0), |(b, w), c| match c {
        'B' => (b + 1, w),
        _ => (b, w + 1),
    });

    if b == w {
        println!("1");
    } else {
        println!("0");
    }
}
