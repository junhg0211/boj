use std::io::stdin;

const DIVISOR: i64 = 1000000007;

fn mul(a: i64, b: i64) -> i64 {
    let result = (a % DIVISOR) * (b % DIVISOR) % DIVISOR;
    if result < 0 {
        return result + DIVISOR;
    }
    return result;
}

fn power(a: i64, mut b: i64) -> i64 {
    let mut multiplier = a;
    let mut result = 1;
    while b > 0 {
        if b & 1 == 1 {
            result *= multiplier;
            result %= DIVISOR;
        }
        b >>= 1;
        multiplier *= multiplier;
        multiplier %= DIVISOR;
    }

    return result;
}

fn invmod(of: i64, when: i64) -> i64 {
    return power(of, when-2);
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let dice_count = input.trim().parse::<usize>().unwrap();

    let mut result = 0;

    for _ in 0..dice_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap());

        let divisor = iter.next().unwrap();
        let sum = iter.next().unwrap();
        // let fraction = (sum, divisor);
        let fraction = mul(sum, invmod(divisor, DIVISOR));

        // result = fraction_add(result, fraction);
        result = (result + fraction) % DIVISOR;
    }

    // let result = mul(result.0, invmod(result.1, DIVISOR));
    println!("{}", result);
}
