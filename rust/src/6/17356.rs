use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();

    let m = (b - a) as f64 / 400.0;
    let ans = 1.0 / (1.0 + 10.0f64.powf(m));

    println!("{}", ans);
}
