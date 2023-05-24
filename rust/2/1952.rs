use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let height = numbers[0];
    let width = numbers[1];

    println!("{}", match height > width {
        true => 1 + (width-1)*2,
        false => (height-1)*2,
    });
}
