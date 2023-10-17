use std::io::stdin;

static DIGITS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace();
    let mut number = iter.next().unwrap().parse::<u32>().unwrap();
    let digits = iter.next().unwrap().parse::<u32>().unwrap();

    let mut stack = Vec::new();
    while number > 0 {
        let letter = DIGITS.chars().nth((number % digits) as usize).unwrap();
        stack.push(letter);

        number /= digits;
    }

    loop {
        match stack.pop() {
            Some(thing) => print!("{}", thing),
            None => {
                println!();
                break;
            },
        }
    }
}
