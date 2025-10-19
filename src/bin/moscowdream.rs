use std::io;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = String::new();
    io::BufRead::read_line(&mut handle, &mut input)?;

    println!("{}", solve(&input));
    Ok(())
}

fn solve(input: &str) -> &'static str {
    let nn = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>();
    if nn[0] < 1 || nn[1] < 1 || nn[2] < 1 || nn[0] + nn[1] + nn[2] < nn[3] || nn[3] < 3 {
        "NO"
    } else {
        "YES"
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve("0 3 3 5"), "NO");
        assert_eq!(solve("4 10 6 13"), "YES");
    }
}
