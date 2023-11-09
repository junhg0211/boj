use std::io::stdin;
use std::cmp::min;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut sides = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    sides.sort();

    let result = sides[0] + sides[1] + min(sides[2], sides[0] + sides[1] - 1);
    println!("{}", result);
}
