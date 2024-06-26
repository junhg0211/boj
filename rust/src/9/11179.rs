use std::{collections::VecDeque, io::stdin};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut number = input.trim().parse::<u32>().unwrap();

    let mut bits = VecDeque::new();
    while number > 0 {
        bits.push_front(number % 2);
        number /= 2
    }

    let mut result = 0;
    while !bits.is_empty() {
        result *= 2;
        result += bits.pop_back().unwrap();
    }

    println!("{}", result);
}
