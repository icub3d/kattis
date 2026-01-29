use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut ss = s.lines();

    let t = ss.next().unwrap().parse::<usize>().unwrap();

    for _ in 0..t {
        let n = ss.next().unwrap().parse::<usize>().unwrap();

        let votes = (0..n)
            .map(|_| ss.next().unwrap().parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let sum = votes.iter().sum::<usize>();
        let max = *votes.iter().max().unwrap();

        let winners = votes
            .iter()
            .enumerate()
            .filter(|&(_, v)| *v == max)
            .collect::<Vec<_>>();

        if winners.len() > 1 {
            println!("no winner");
            continue;
        }

        if max > sum / 2 {
            println!("majority winner {}", winners[0].0 + 1);
        } else {
            println!("minority winner {}", winners[0].0 + 1);
        }
    }
}
