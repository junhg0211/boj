use std::io::stdin;
use std::collections::HashMap;

fn main() {
    // get numbers
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let numbers: Vec<u32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // calculate the count one by one
    let mut count: HashMap<u32, u32> = HashMap::new();
    for i in 1..=numbers[0] {
        for j in 1..=numbers[1] {
            for k in 1..=numbers[2] {
                let sum = i + j + k;
                let counter = count.entry(sum).or_insert(0);
                *counter += 1;
            }
        }
    }

    // find the max count
    let mut max_count = 0;
    let mut max_sum = 0;
    for (sum, counter) in count {
        if counter > max_count {
            max_count = counter;
            max_sum = sum;
        } else if counter == max_count {
            if sum < max_sum {
                max_sum = sum;
            }
        }
    }

    // print the result
    println!("{}", max_sum);
}
