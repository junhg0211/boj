use std::io::stdin;
use std::cmp::max;

fn tick() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut iter = input.trim().split_whitespace();

    let a = iter.next().unwrap().chars().rev();
    let b = iter.next().unwrap().chars().rev();

    let mut carry = false;
    let mut result = String::new();
    let length = max(a.clone().collect::<String>().len(), b.clone().collect::<String>().len());

    for i in 0..length {
        let x = a.clone().nth(i).unwrap_or('0') == '1';
        let y = b.clone().nth(i).unwrap_or('0') == '1';

        let sum = x ^ y ^ carry;
        carry = (x & y) | (x & carry) | (y & carry);

        result.push(if sum { '1' } else { '0' });
    }

    if carry == true {
        result.push('1');
    }

    result = result.trim_end_matches('0').chars().rev().collect::<String>();

    if result.len() == 0 {
        println!("0");
    } else {
        println!("{}", result);
    }
}

fn main() {
    let count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    for _ in 0..count {
        tick();
    }
}
