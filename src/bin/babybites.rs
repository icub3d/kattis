use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    for m in s.lines().skip(1) {
        let mut good = true;
        for (i, p) in m.split_whitespace().enumerate() {
            match p.parse::<usize>() {
                Ok(n) => {
                    if n == i + 1 {
                        continue;
                    }
                }
                Err(_) => continue,
            }
            good = false;
            break;
        }

        if good {
            println!("makes sense");
        } else {
            println!("something is fishy");
        }
    }
}
