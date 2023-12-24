use std::io::stdin;

fn main() {
    // get numbers
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let numbers: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let now = numbers[0];
    let target = numbers[1];
    let delta = numbers[2];

    // calculate
    let result = ((target - now) / delta as f64).ceil();
    if result > 0.0 {
        println!("{}", result);
    } else {
        println!("0");
    }
}
