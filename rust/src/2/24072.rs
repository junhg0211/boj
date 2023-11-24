use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());

    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    let c = iter.next().unwrap();

    println!("{}", match a <= c && c < b {
        true => 1,
        false => 0
    });
}
