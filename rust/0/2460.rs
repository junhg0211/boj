use std::io::stdin;
use std::cmp::max;

fn main() {
    let mut input = String::new();

    let mut now: u32 = 0;
    let mut result: u32 = 0;

    for _ in 0..10 {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let numbers: Vec<u32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let out = numbers[0];
        let in_ = numbers[1];

        now -= out;
        now += in_;

        result = max(now, result);
    }

    println!("{}", result);
}
