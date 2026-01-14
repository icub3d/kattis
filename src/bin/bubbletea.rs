use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut ss = s.lines();
    ss.next();

    let teas = ss
        .next()
        .unwrap()
        .split_whitespace()
        .map(|t| t.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    ss.next();
    let toppings = ss
        .next()
        .unwrap()
        .split_whitespace()
        .map(|t| t.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut min = usize::MAX;
    for t in teas {
        // MIN! (pick one)
        let c = ss
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .skip(1)
            .map(|j| t + toppings[j - 1])
            .min()
            .unwrap();

        min = min.min(c);
    }

    let money = ss.next().unwrap().parse::<usize>().unwrap();

    let total = money / min;
    // sub/add wrong (professor needs one)
    println!("{}", total.saturating_sub(1));
}
