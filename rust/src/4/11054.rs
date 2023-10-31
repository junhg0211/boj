use std::io::stdin;
use std::cmp::max;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let number_count = input.trim().parse::<usize>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    // left-to-right lengths
    let mut lrlengths = Vec::new();
    for i in 0..number_count {
        let mut length = 1;
        for j in 0..i {
            if numbers[j] >= numbers[i] {
                continue;
            }

            length = max(lrlengths[j] + 1, length);
        }

        lrlengths.push(length);
    }

    // right-to-left lengths
    let mut rllengths = Vec::new();
    for i in 0..number_count {
        let index = number_count-1 - i;
        let mut length = 1;
        for j in index+1..number_count {
            if numbers[index] <= numbers[j] {
                continue;
            }

            length = max(rllengths[number_count-1 - j] + 1, length);
        }

        rllengths.push(length);
    }

    // count and get max length
    let mut max_length = 0;
    for i in 0..number_count {
        max_length = max(
            max_length, rllengths[number_count-1 - i] + lrlengths[i] - 1
        );
    }

    println!("{}", max_length);
}
