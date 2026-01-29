use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut ss = s.lines();

    let n = ss.next().unwrap().parse::<usize>().unwrap();

    for _ in 0..n {
        let k = ss.next().unwrap().parse::<usize>().unwrap();

        let r = ss.next().unwrap();

        let menu = (0..k).map(|_| ss.next().unwrap()).collect::<Vec<_>>();

        if menu.contains(&"pea soup") && menu.contains(&"pancakes") {
            println!("{r}");
            return;
        }
    }

    println!("Anywhere is fine I guess");
}
