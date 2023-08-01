use std::io::stdin;

fn main() {
    let mut numbers = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
    };
    numbers.sort();

    println!("{}", numbers[1] + numbers[2]);
}
