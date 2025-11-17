use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let names = s.lines().skip(1).collect::<Vec<_>>();

    // We need to verify that the direction is the same for all.
    let direction = names[0].cmp(names[1]);
    for w in names.windows(2) {
        if direction != w[0].cmp(w[1]) {
            println!("NEITHER");
            return;
        }
    }

    // Otherwise print the direction.
    match direction {
        std::cmp::Ordering::Greater => println!("DECREASING"),
        std::cmp::Ordering::Less => println!("INCREASING"),
        _ => panic!("doh!"),
    }
}
