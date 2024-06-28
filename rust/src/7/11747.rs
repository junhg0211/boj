use std::{collections::VecDeque, io::stdin};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<usize>().unwrap();

    let mut numbers = Vec::new();
    loop {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let nums = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap());

        numbers.extend(nums);

        if numbers.len() == count {
            break;
        }
    }

    let mut now = 0;
    loop {
        // get digits of number
        let mut digits = VecDeque::new();
        let mut i = now;
        if i == 0 {
            digits.push_front(0);
        } else {
            while i > 0 {
                digits.push_front(i % 10);
                i /= 10;
            }
        }

        // find intersection
        let mut exist = false;
        for i in 0..=count - digits.len() {
            let mut matches = true;
            for j in 0..digits.len() {
                let digit = digits[j];
                if digit != numbers[i + j] {
                    matches = false;
                    break;
                }
            }

            if matches {
                exist = true;
                break;
            }
        }

        // if not found, return them
        if !exist {
            break;
        }

        // increase number if found
        now += 1;
    }

    println!("{}", now);
}
