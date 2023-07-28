use std::io::stdin;

fn main() {
    let count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    for _ in 0..count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut numbers = input
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        numbers.sort();

        println!("{}", numbers[7]);
    }
}
