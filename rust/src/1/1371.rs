use std::cmp::max;
use std::collections::HashMap;
use std::io::stdin;

fn main() {
    // count letters
    let mut counts: HashMap<char, u32> = HashMap::new();
    let mut max_count = 0;

    loop {
        let mut input = String::new();
        let letters = stdin().read_line(&mut input).unwrap();

        if letters == 0 {
            break;
        }

        for letter in input.trim().chars() {
            if letter == ' ' {
                continue;
            }

            if counts.contains_key(&letter) {
                max_count = max(
                    {
                        let count = counts.get(&letter).unwrap() + 1;
                        counts.insert(letter, count);
                        count
                    }, max_count);
            } else {
                counts.insert(letter, 1);
                max_count = max(1, max_count);
            }
        }
    }

    // get max letters
    let mut letters: Vec<char> = Vec::new();
    for key in counts.keys() {
        if *counts.get(key).unwrap() == max_count {
            letters.push(*key);
        }
    }

    // print result
    letters.sort();
    for letter in letters.iter() {
        print!("{}", letter);
    }
}