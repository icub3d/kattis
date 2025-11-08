use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let (nums, order) = s
        .split_once("\n")
        .map(|(n, o)| {
            let mut n = n
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            n.sort();

            (
                n,
                o.trim()
                    .chars()
                    .map(|c| c as usize - 'A' as usize)
                    .collect::<Vec<_>>(),
            )
        })
        .unwrap();

    println!(
        "{}",
        order
            .iter()
            .map(|i| nums[*i].to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
