use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap());
    let a1 = iter.next().unwrap();
    let a0 = iter.next().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let c = input.trim().parse::<i32>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n0 = input.trim().parse::<i32>().unwrap();

    if a1 <= c && (a1 == c && a0 <= 0 || a1 < c && a0 <= n0 * (c-a1)) {
        println!("1");
    } else {
        println!("0");
    }
}
