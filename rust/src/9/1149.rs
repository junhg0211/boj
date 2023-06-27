use std::cmp::min;
use std::io::stdin;

fn main() {
    // get count
    let count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().parse::<u32>().unwrap()
    };

    // calculate result
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    for _ in 0..count {
        let numbers: Vec<_> = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            input.trim()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect()
        };

        (a, b, c) = (
            numbers[0] + min(b, c),
            numbers[1] + min(a, c),
            numbers[2] + min(a, b)
        );
    }

    // print result
    println!("{}", min(min(a, b), c));
}