use std::io::stdin;
use std::cmp::min;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let numbers: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let n = numbers[0];
    let m = numbers[1];
    let k = numbers[2];

    println!("{}", min(m,k) + min(n-m, n-k));
}
