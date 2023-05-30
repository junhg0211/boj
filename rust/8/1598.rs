use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap() - 1)
        .collect();

    let dy = (numbers[0] / 4 - numbers[1] / 4).abs();
    let dx = (numbers[0] % 4 - numbers[1] % 4).abs();

    println!("{}", dy + dx);
}
