use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let mut shares = 0.0f64;
    let mut avg = 0.0f64;

    for m in s.lines() {
        let line = m.split_whitespace().collect::<Vec<_>>();

        match line[0] {
            "buy" => {
                let count = line[1].parse::<f64>().unwrap();
                let price = line[2].parse::<f64>().unwrap();

                avg = (shares * avg + count * price) / (shares + count);
                shares += count;
            }

            "sell" => {
                let count = line[1].parse::<f64>().unwrap();
                shares -= count;
            }

            "split" => {
                let n = line[1].parse::<f64>().unwrap();
                shares *= n;
                avg /= n;
            }

            "merge" => {
                let n = line[1].parse::<f64>().unwrap();
                shares = (shares / n).floor();
                avg *= n;
            }

            _ => {
                let mut sell = line[1].parse::<f64>().unwrap();
                let profit = sell - avg;

                if profit > 0.0 {
                    sell -= profit * 0.3;
                }
                println!("{:.8}", shares * sell);
                break;
            }
        }
    }
}
