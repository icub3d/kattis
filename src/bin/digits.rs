use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    fn digits(mut n: usize) -> usize {
        if n == 0 {
            return 1;
        }

        let mut d = 0;
        while n > 0 {
            n /= 10;
            d += 1;
        }
        d
    }

    for m in s.lines() {
        if m == "END" {
            break;
        } else if m == "1" {
            println!("1");
            continue;
        }

        let mut count = 2;
        let mut cur = m.len();

        while cur != digits(cur) {
            cur = digits(cur);
            count += 1;
        }

        println!("{count}");
    }
}
