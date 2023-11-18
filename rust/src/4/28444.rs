use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap());
    let h = iter.next().unwrap();
    let i = iter.next().unwrap();
    let a = iter.next().unwrap();
    let r = iter.next().unwrap();
    let c = iter.next().unwrap();

    println!("{}", h*i - a*r*c);
}
