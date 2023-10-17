use std::io::stdin;

static DIGITS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace();
    let number = iter.next().unwrap().to_owned();
    let digits = iter.next().unwrap().parse::<u32>().unwrap();

    let mut result = 0;
    for i in 0..number.len() {
        let now_char =  number.chars().nth(i).unwrap();
        let mut position = 0;
        loop {
            if DIGITS.chars().nth(position).unwrap() == now_char {
                break;
            }

            position += 1;
        }

        result *= digits;
        result += position as u32;
    }

    println!("{}", result);
}
