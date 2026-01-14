use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut ss = s.lines();
    let n = ss.next().unwrap().parse::<usize>().unwrap();
    let d = ss.next().unwrap();

    let f = |c: usize| match c {
        0 => format!("no more bottles of {d}"),
        1 => format!("1 bottle of {d}"),
        _ => format!("{c} bottles of {d}"),
    };

    for i in (1..=n).rev() {
        let (cur, next) = (f(i), f(i - 1));
        let p = if i == 1 { "it" } else { "one" };

        println!("{cur} on the wall, {cur}.");
        print!("Take {p} down, pass it around, {next}");
        if i > 1 {
            println!(" on the wall.");
            println!();
        } else {
            println!(".");
        }
    }
}
