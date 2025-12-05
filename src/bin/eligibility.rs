use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    for m in s.lines().skip(1) {
        let pp = m.split_whitespace().collect::<Vec<_>>();
        let (name, studies, dob, courses) = (
            pp[0],
            pp[1][..4].parse::<usize>().unwrap(),
            pp[2][..4].parse::<usize>().unwrap(),
            pp[3].parse::<usize>().unwrap(),
        );

        if studies >= 2010 || dob >= 1991 {
            println!("{name} eligible");
        } else if courses >= 41 {
            println!("{name} ineligible");
        } else {
            println!("{name} coach petitions");
        }
    }
}
