use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    numbers.sort();

    let a = numbers[0];
    let b = numbers[1];
    let c = numbers[2];

    if a == b || b == c || a + b == c || a == b + c {
        println!("S");
    } else {
        println!("N");
    }
}
