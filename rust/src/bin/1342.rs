use std::collections::HashMap;
use std::io::stdin;

fn main() {
    // get string
    let mut string = String::new();
    let string = {
        stdin().read_line(&mut string).unwrap();
        string.trim()
    };

    // count letter count and sort the counts
    let mut counts: HashMap<char, u32> = HashMap::new();
    for letter in string.chars() {
        if counts.contains_key(&letter) {
            let count = counts.get_mut(&letter).unwrap();
            *count += 1;
        } else {
            counts.insert(letter, 1);
        }
    }

    let mut counts: Vec<_> = counts.values().map(|x| *x).collect();
    counts.sort();

    // calculate possibles
    let mut result: u32 = 1;
    let mut multiples = 0;
    for i in 1..=counts.len() {
        result *= i as u32;

        if counts[i-1] >= 2 {
            let n = (counts.len() - 1 + multiples) as u32;
            let r = counts[i-1] - 1;
            result *= ncr(n, r);
            println!("{} {} {}", i, n, r);

            multiples += 1;
        }
    }

    println!("{}", result);
}

fn ncr(n: u32, r: u32) -> u32 {
    if n < r {
        0
    } else {
        (1..=n).product::<u32>() / (1..=r).product::<u32>() / (1..=(n-r)).product::<u32>()
    }
}