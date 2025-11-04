use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    for prev in s.lines() {
        if prev == "END" {
            break;
        }
        let mut count = 1;
        let mut prev = prev.to_string();
        let mut cur = format!("{}", prev.len());
        while prev != cur {
            prev = cur.clone();
            cur = format!("{}", prev.len());
            count += 1;
        }
        println!("{count}");
    }
}
