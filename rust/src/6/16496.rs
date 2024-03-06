use std::cmp::max;
use std::io::stdin;

fn lt(a: &Vec<char>, b: &Vec<char>) -> bool {
    for i in 0..max(a.len(), b.len()) {
        if i >= a.len() {
            // 1110 < 1
            for j in i..b.len() {
                if a[j % a.len()] > b[j] {
                    return false;
                }
                if a[j % a.len()] < b[j] {
                    return true;
                }
            }

            // 101 < 1011, 373 > 3733
            for j in 0..a.len() - 1 {
                if a[j] > a[j + 1] {
                    return true;
                }
                if a[j] < a[j + 1] {
                    return false;
                }
            }

            // whatever
            return false;
        }

        if i >= b.len() {
            // 1110 < 1
            for j in i..a.len() {
                if a[j] > b[j % b.len()] {
                    return false;
                }
                if a[j] < b[j % b.len()] {
                    return true;
                }
            }

            // 101 < 1011, 373 > 3733
            for j in 0..b.len() - 1 {
                if b[j] > b[j + 1] {
                    return false;
                }
                if b[j] < b[j + 1] {
                    return true;
                }
            }

            // whatever
            return true;
        }

        if a[i] < b[i] {
            return true;
        }

        if a[i] > b[i] {
            return false;
        }
    }
    return false;
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<usize>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    // bubble sort
    for i in 0..count {
        let i = count - i - 1;
        for j in 0..i {
            if lt(&numbers[j], &numbers[i]) {
                continue;
            }

            // swap
            (numbers[i], numbers[j]) = (numbers[j].clone(), numbers[i].clone());
        }
    }

    // print
    let mut start = true;
    for i in 0..count {
        let i = count - i - 1;
        let number = &numbers[i];

        for &letter in number {
            if start && letter == '0' {
                continue;
            }

            start = false;
            print!("{}", letter);
        }
        // print!(" ");
    }

    if start {
        println!("0");
    } else {
        println!();
    }
}
