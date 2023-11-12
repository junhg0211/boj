use std::io::stdin;
use std::cmp::min;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let number_count = iter.next().unwrap();
    let min_sum = iter.next().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut i = 0;
    let mut j = 0;
    let mut sum = 0;
    let mut result = usize::MAX;
    while j < number_count || sum >= min_sum {
        if sum < min_sum && j < number_count {
            sum += numbers[j];
            j += 1;
        } else {
            sum -= numbers[i];
            i += 1;
        }

        if sum >= min_sum {
            result = min(result, j-i);
        }
    }

    if result == usize::MAX {
        println!("0");
    } else {
        println!("{}", result);
    }
}
