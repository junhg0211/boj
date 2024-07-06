use std::{collections::HashSet, io::stdin};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<usize>().unwrap();

    let mut result = 0;
    let mut greetings = HashSet::new();
    for _ in 0..count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let log = input.trim().to_string();

        if log == "ENTER" {
            greetings.clear();
            continue;
        }

        if greetings.contains(&log) {
            continue;
        }

        greetings.insert(log);
        result += 1;
    }

    println!("{}", result);
}
