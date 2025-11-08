use std::{
    collections::HashSet,
    io::{Read, stdin},
};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();
    let mut ll = s.lines();
    let (max_y, max_x) = ll
        .next()
        .unwrap()
        .split_once(' ')
        .map(|(r, c)| (r.parse::<i32>().unwrap(), c.parse::<i32>().unwrap()))
        .unwrap();

    let grid = ll.map(|l| l.chars().collect()).collect::<Vec<Vec<_>>>();
    let mut cur = Point::new(0, 0);
    let mut seen = HashSet::new();
    seen.insert(cur);
    let mut steps = 0;
    loop {
        cur = match grid[cur.y as usize][cur.x as usize] {
            'S' => Point::new(cur.x, cur.y + 1),
            'N' => Point::new(cur.x, cur.y - 1),
            'E' => Point::new(cur.x + 1, cur.y),
            'W' => Point::new(cur.x - 1, cur.y),
            'T' => {
                println!("{steps}");
                break;
            }
            _ => panic!("doh"),
        };
        if cur.y < 0 || cur.y >= max_y || cur.x < 0 || cur.x >= max_x {
            println!("Out");
            break;
        }
        if !seen.insert(cur) {
            println!("Lost");
            break;
        }
        steps += 1;
    }
}
