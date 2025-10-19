use std::io::{self, BufRead, stdin};

fn main() -> io::Result<()> {
    let input = stdin().lock().lines().collect::<Result<Vec<String>, _>>()?;
    println!("{}", solve(&input));
    Ok(())
}

fn solve(input: &[String]) -> String {
    let (_, p) = input[0].split_once(' ').unwrap();
    p.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "2 1
carrots?
bunnies
";
        let input = input
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
        assert_eq!(solve(&input), "1");

        let input = "1 5
sovl problmz
";
        let input = input
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
        assert_eq!(solve(&input), "5");
    }
}
