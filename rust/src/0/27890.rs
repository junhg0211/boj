use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());

    let mut height = iter.next().unwrap();
    let n = iter.next().unwrap();

    for _ in 0..n {
        height = if height % 2 == 0 {
            (height/2) ^ 6
        } else {
            (2 * height) ^ 6
        };
    }

    println!("{}", height);
}
