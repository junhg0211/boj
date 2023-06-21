use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let count = input.trim().parse::<u32>().unwrap();
    let mut position = 1;

    for _ in 0..count {
        input.clear();
        stdin().read_line(&mut input).unwrap();

        let numbers: Vec<u32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        if numbers[0] == position {
            position = numbers[1];
        } else if numbers[1] == position {
            position = numbers[0];
        }
    }

    println!("{}", position);
}
