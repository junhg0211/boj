use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap());
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    let c = iter.next().unwrap();
    let d = iter.next().unwrap();
    let e = iter.next().unwrap();
    let f = iter.next().unwrap();

    for x in -999..=999 {
        for y in -999..=999 {
            if a*x + b*y == c && d*x + e*y == f {
                println!("{} {}", x, y);
                return;
            }
        }
    }
}
