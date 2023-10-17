use std::io::stdin;
use std::cmp::max;

fn main() {
    let mut words = Vec::new();
    let mut max_length = 0;
    for _ in 0..5 {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        words.push(input.trim().to_owned());
        max_length = max(max_length, input.len());
    }

    for i in 0..max_length {
        for j in 0..5 {
            match words[j].chars().nth(i) {
                Some(thing) => print!("{}", thing),
                None => continue,
            }
        }
    }
    println!();
}
