use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let message = input.trim();

    let row = "`1234567890-=QWERTYUIOP[]\\ASDFGHJKL;'ZXCVBNM,./";

    for letter in message.chars() {
        match row.find(letter) {
            Some(index) => print!("{}", row.chars().nth(index - 1).unwrap()),
            None => print!("{}", letter)
        }
    }

    println!();
}