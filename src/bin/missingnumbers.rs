use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let nn = s.lines().skip(1).map(|n| n.parse::<usize>().unwrap());

    let mut exp = 1;
    let mut missed = false;

    for n in nn {
        while exp < n {
            println!("{exp}");
            exp += 1;
            missed = true;
        }

        exp = n + 1;
    }

    if !missed {
        println!("good job");
    }
}
