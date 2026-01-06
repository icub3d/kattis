use std::{
    collections::{HashMap, HashSet},
    io::{Read, stdin},
};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut solved = HashSet::new();
    let mut fails = HashMap::new();

    let (mut count, mut score) = (0, 0);

    for m in s.lines() {
        if m == "-1" {
            break;
        }
        let pp = m.split_whitespace().collect::<Vec<_>>();

        let (t, p, v) = (
            pp[0].parse::<usize>().unwrap(),
            pp[1].chars().next().unwrap(),
            pp[2] == "right",
        );

        if !solved.contains(&p) {
            if v {
                solved.insert(p);
                count += 1;
                score += t + fails.get(&p).unwrap_or(&0) * 20;
            } else {
                *fails.entry(p).or_default() += 1;
            }
        }
    }

    println!("{count} {score}");
}
