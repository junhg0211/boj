use std::io::stdin;

fn get_gcd(mut a: u32, mut b: u32) -> u32 {
    if a < b {
        return get_gcd(b, a);
    }

    while b > 0 {
        (a, b) = (b, a%b);
    }

    return a;
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    // let (a, b) = if a < b { (a, b) } else { (b, a) };

    let gcd = get_gcd(a, b);
    let result = a/gcd + b/gcd - 1;

    println!("{}", result * gcd);
}
