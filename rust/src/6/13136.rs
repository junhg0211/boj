use std::io::stdin;

fn main() {
    // get input
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let numbers: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let width = numbers[0];
    let height = numbers[1];
    let size = numbers[2];

    // calculate
    let result = (width / size).ceil() * (height / size).ceil();

    println!("{}", result);
}
