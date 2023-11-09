use std::io::stdin;
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let string = input.trim();

    let mut substrings = HashSet::new();
    for length in 1..=string.len() {
        for i in 0..=string.len() - length {
            let substring = &string[i..i+length];
            substrings.insert(substring);
        }
    }

    println!("{}", substrings.len());
}
