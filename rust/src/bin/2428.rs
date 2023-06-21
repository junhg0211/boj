use std::io::stdin;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();

    // get number count
    stdin().read_line(&mut input).unwrap();

    // get numbers
    input.clear();
    stdin().read_line(&mut input).unwrap();
    let numbers: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    // get counts
    let mut counts: HashMap<u32, u32> = HashMap::new();
    for number in numbers.iter() {
        if counts.contains_key(&number) {
            let count = counts.get_mut(&number).unwrap();
            *count += 1;
        } else {
            counts.insert(*number, 1);
        }
    }

    // calculate combinations count
    let mut result = 0;
    for now_key in counts.keys() {
        for next_key in counts.keys() {
            if !(*now_key as f64 >= 0.9 * *next_key as f64) {
                continue;
            }
            if !(now_key < next_key) {
                continue;
            }

            result += counts[now_key]* counts[next_key];
        }
    }

    // print result
    println!("{}", result);
}
