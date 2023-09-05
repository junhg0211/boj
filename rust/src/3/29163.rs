use std::io::stdin;

fn main() {
    let _ = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>()
    };

    let numbers = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    };

    let mut odds = 0;
    let mut evens = 0;
    for number in numbers {
        match number % 2 {
            0 => evens += 1,
            1 => odds += 1,
            _ => (),
        }
    }

    match odds < evens {
        true => println!("Happy"),
        false => println!("Sad"),
    }
}
