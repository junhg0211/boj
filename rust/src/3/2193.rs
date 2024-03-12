use std::cmp::max;
use std::io::stdin;

const BASE: u32 = 1000000000;

fn add(a: &Vec<u32>, b: &Vec<u32>) -> Vec<u32> {
    let mut result = Vec::new();

    let mut carry = 0;
    for i in 0..max(a.len(), b.len()) {
        let number =
            if i < a.len() { a[i] } else { 0 } + if i < b.len() { b[i] } else { 0 } + carry;

        carry = number / BASE;
        result.push(number % BASE);
    }

    if carry > 0 {
        result.push(carry);
    }

    result
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<u32>().unwrap();

    let mut dp = (vec![0], vec![1]);
    for _ in 1..count {
        dp = (add(&dp.0, &dp.1), dp.0);
    }

    let mut result = add(&dp.0, &dp.1);
    let mut first = true;
    while let Some(thing) = result.pop() {
        if first {
            print!("{}", thing);
            first = false;
        } else {
            print!("{:09}", thing);
        }
    }
    println!();
}
