use std::io::stdin;
use std::cmp;

fn main() {
    let mut input = String::new();

    let mut max = 0;
    let mut bus = 0;
    for _ in 0..4 {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let numbers: Vec<u32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        bus -= numbers[0];
        bus += numbers[1];

        max = cmp::max(max, bus);
    }

    println!("{}", max);
}
