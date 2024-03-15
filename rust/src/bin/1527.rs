use std::io::stdin;

fn next(mut number: u64) -> u64 {
    if number == 0 {
        return 4;
    }

    let mut result = 0;

    let mut stack = Vec::new();
    let mut ok = false;
    let mut carry = 0;
    while number > 0 || carry > 0 {
        if carry > 0 {
            number += 1;
            carry = 0;
        }

        if ok {
            if number % 10 <= 4 {
                stack.push(4);
            } else if number % 10 <= 7 {
                stack.push(7);
            } else {
                stack.push(4);
                carry = 1;
            }
        } else {
            carry = 0;
            if number % 10 < 4 {
                stack.push(4);
            } else if number % 10 < 7 {
                if number < 10 || number / 10 % 10 == 4 || number / 10 % 10 == 7 {
                    stack.push(7);
                } else {
                    stack.push(4);
                    carry = 1;
                }
            } else {
                stack.push(4);
                carry = 1;
            }
            ok = true;
        }

        number /= 10;
    }

    while let Some(thing) = stack.pop() {
        result *= 10;
        result += thing;
    }

    return result;
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap());

    let mut start = next(iter.next().unwrap() - 1);
    let end = iter.next().unwrap();

    let mut result = 0;
    while start <= end {
        // println!("{}", start);
        start = next(start);
        result += 1;
    }

    println!("{}", result);
}

