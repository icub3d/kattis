use std::{
    collections::HashMap,
    io::{Read, stdin},
};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let scores: HashMap<&str, Vec<(usize, bool)>> = s.lines().fold(HashMap::new(), |mut acc, l| {
        let pp = l.split_whitespace().collect::<Vec<_>>();
        if pp.len() != 3 {
            return acc;
        }
        let time = pp[0].parse::<usize>().unwrap();
        acc.entry(pp[1])
            .or_default()
            .push((time, matches!(pp[2], "right")));
        acc
    });

    let (i, t) = scores
        .values()
        .map(|attempts| {
            for (penalties, &(time, attempt)) in attempts.iter().enumerate() {
                if attempt {
                    return (1, time + 20 * penalties);
                }
            }
            (0, 0)
        })
        .fold((0, 0), |(ai, at), (i, t)| (ai + i, at + t));

    println!("{i} {t}");
}
