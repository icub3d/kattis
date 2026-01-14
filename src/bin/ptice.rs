use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let quiz = s.lines().nth(1).unwrap();

    let patterns = [("Adrian", "ABC"), ("Bruno", "BABC"), ("Goran", "CCAABB")];

    let scores = patterns
        .iter()
        .map(|&(n, c)| {
            (
                n,
                quiz.chars()
                    .zip(c.chars().cycle())
                    .filter(|(a, g)| a == g)
                    .count(),
            )
        })
        .collect::<Vec<_>>();

    let max = scores.iter().max_by_key(|(_, s)| s).unwrap().1;

    println!("{max}");
    scores
        .iter()
        .filter(|(_, s)| *s == max)
        .for_each(|(n, _)| println!("{n}"));
}
