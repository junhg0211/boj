use std::io::stdin;
use std::collections::HashMap;

const DIVISOR: usize = 900528;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let letters = input.trim().chars();

    let mut indexes = HashMap::new();
    for (i, letter) in letters.enumerate() {
        indexes.insert(letter, i+1);
    }
    let letter_count = indexes.len();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let message = input.trim().chars();
    let mut result = 0;
    for letter in message {
        result *= letter_count;
        result += indexes.get(&letter).unwrap();
        result %= DIVISOR;
    }

    println!("{}", result);
}
