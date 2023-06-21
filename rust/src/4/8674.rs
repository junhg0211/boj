use std::io::stdin;
use std::cmp;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let numbers: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    if numbers[0] % 2 == 0 || numbers[1] % 2 == 0 {
        println!("0");
        return;
    }

    println!("{}", cmp::min(numbers[0], numbers[1]));
}
