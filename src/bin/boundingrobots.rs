use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut lines = s.lines();

    while let Some(l) = lines.next() {
        if l == "0 0" {
            break;
        }

        // OFF BY ONE!!!
        let (w, l) = l
            .split_once(' ')
            .map(|(l, r)| {
                (
                    l.parse::<isize>().unwrap() - 1,
                    r.parse::<isize>().unwrap() - 1,
                )
            })
            .unwrap();

        let n = lines.next().unwrap().parse::<usize>().unwrap();

        let (mut rx, mut ry) = (0, 0); //robot
        let (mut ax, mut ay) = (0, 0); //actual

        for _ in 0..n {
            let (dir, dist) = lines
                .next()
                .unwrap()
                .split_once(' ')
                .map(|(l, r)| (l, r.parse::<isize>().unwrap()))
                .unwrap();

            match dir {
                "u" => {
                    ry += dist;
                    ay = (ay + dist).clamp(0, l);
                }
                "d" => {
                    ry -= dist;
                    ay = (ay - dist).clamp(0, l);
                }
                "r" => {
                    rx += dist;
                    ax = (ax + dist).clamp(0, w);
                }
                _ => {
                    rx -= dist;
                    ax = (ax - dist).clamp(0, w);
                }
            }
        }

        println!("Robot thinks {rx} {ry}");
        println!("Actually at {ax} {ay}");
        println!();
    }
}
