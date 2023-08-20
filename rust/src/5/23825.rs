use std::io::stdin;
use std::cmp::min;

fn main() {
    let (a, b) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input.trim().split_whitespace();

        (
            iter.next().unwrap().parse::<u32>().unwrap(),
            iter.next().unwrap().parse::<u32>().unwrap(),
        )
    };

    println!("{}", min(a/2, b/2));
}
