use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut ss = s.lines();
    let t = ss
        .next()
        .unwrap()
        .split_once(' ')
        .map(|(_, t)| t.parse::<usize>().unwrap())
        .unwrap();

    let a = ss
        .next()
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    match t {
        1 => println!("7"),
        2 => {
            if a[0] > a[1] {
                println!("Bigger");
            } else if a[0] == a[1] {
                println!("Equal");
            } else {
                println!("Smaller");
            }
        }
        3 => {
            let mut f = a[0..3].iter().collect::<Vec<_>>();
            f.sort();
            println!("{}", a[1]);
        }
        4 => println!("{}", a.iter().sum::<usize>()),
        5 => println!("{}", a.iter().filter(|&a| a % 2 == 0).sum::<usize>()),
        6 => {
            println!(
                "{}",
                a.iter()
                    .map(|a| char::from_u32(97 + (a % 26) as u32).unwrap())
                    .collect::<String>()
            )
        }
        7 => {
            let mut i = 0;
            let mut seen = vec![false; a.len()];
            loop {
                if seen[i] {
                    println!("Cyclic");
                    break;
                }
                seen[i] = true;
                i = a[i];
                if i >= a.len() {
                    println!("Out");
                    break;
                } else if i == a.len() - 1 {
                    println!("Done");
                }
            }
        }
        _ => panic!("unknown"),
    }
}
