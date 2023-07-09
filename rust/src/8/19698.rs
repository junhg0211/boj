use std::io::stdin;
use std::cmp::min;

fn main() {
    // get numbers
    let (n, w, h, l) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let numbers = input
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        (numbers[0], numbers[1], numbers[2], numbers[3])
    };

    // print output
    println!("{}", min(n, (w / l) * (h / l)));
}
