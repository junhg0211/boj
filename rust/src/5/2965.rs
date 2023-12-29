use std::io::stdin;
use std::cmp::max;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace().map(|x| x.parse::<u32>().unwrap());

    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    let c = iter.next().unwrap();

    let ga = b-a-1;
    let gb = c-b-1;

    println!("{}", max(ga, gb));
}
