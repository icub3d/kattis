use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let n = s.trim().parse::<f64>().unwrap();

    println!("{}", (n * (1000. * 5280. / 4854.)).round() as usize);
}
