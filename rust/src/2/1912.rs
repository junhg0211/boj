use std::cmp::max;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<usize>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap());

    let mut dp = 0;
    let mut result = i32::MIN;
    for _ in 0..count {
        let number = iter.next().unwrap();
        dp = max(dp + number, number);
        result = max(result, dp);
    }

    println!("{}", result);
}
