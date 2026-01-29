use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let s = s.trim().as_bytes();
    let mut i = 0;
    while i < s.len() {
        if i + 2 < s.len() && (s[i] != s[i + 1] && s[i + 1] != s[i + 2] && s[i] != s[i + 2]) {
            print!("C");
            i += 3;
        } else {
            print!(
                "{}",
                match s[i] {
                    b'R' => 'S',
                    b'B' => 'K',
                    _ => 'H',
                }
            );
            i += 1;
        }
    }
}
