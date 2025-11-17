use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let first = s.chars().step_by(2);
    let second = s
        .chars()
        .skip(1)
        .step_by(2)
        .map(|c| c.to_digit(10).unwrap());
    let pairs = first.zip(second);

    let (mut a, mut b) = (0, 0);
    for (who, score) in pairs {
        match who {
            'A' => a += score,
            _ => b += score,
        };

        if a >= 11 && a > b && a - b >= 2 {
            println!("A");
            return;
        } else if b >= 11 && b > a && b - a >= 2 {
            println!("B");
            return;
        }
    }
}
