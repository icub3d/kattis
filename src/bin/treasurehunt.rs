use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut lines = s.lines();
    let (mr, mc) = lines
        .next()
        .unwrap()
        .split_once(' ')
        .map(|(r, c)| (r.parse::<isize>().unwrap(), c.parse::<isize>().unwrap()))
        .unwrap();

    let grid = lines.map(|l| l.chars().collect()).collect::<Vec<Vec<_>>>();

    let mut visited = vec![vec![false; mc as usize]; mr as usize];

    let (mut r, mut c) = (0, 0);
    let mut steps = 0;

    loop {
        if r < 0 || c < 0 || r >= mr || c >= mc {
            println!("Out");
            break;
        } else if grid[r as usize][c as usize] == 'T' {
            println!("{steps}");
            break;
        } else if visited[r as usize][c as usize] {
            println!("Lost");
            break;
        }

        visited[r as usize][c as usize] = true;
        match grid[r as usize][c as usize] {
            'N' => r -= 1,
            'S' => r += 1,
            'E' => c += 1,
            'W' => c -= 1,
            _ => unreachable!(),
        };
        steps += 1;
    }
}
