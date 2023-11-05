use std::io::stdin;

const DIVISOR: i64 = 1000000007;

fn mul(a: i64, b: i64) -> i64 {
    let result = (a%DIVISOR) * (b%DIVISOR) % DIVISOR;
    if result < 0 {
        return result + DIVISOR;
    }
    return result;
}

fn eea(t: i64, s: i64) -> (i64, i64) {
    let (q, r) = (s/t, s%t);

    if r == 0 {
        return (1, 0);
    }

    let (ap, app) = eea(r, t);
    let a = (app - mul(q, ap)) % s;
    return (a, ap);
}

fn invmod(of: i64, when: i64) -> i64 {
    return eea(of, when).0;
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    if a < b {
        return gcd(b, a);
    }

    while b != 0 {
        (a, b) = (b, a%b);
    }

    return a;
}

fn fraction_add(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    let gcd_ = gcd(a.1, b.1);
    let numerator = mul(a.0, b.1) / gcd_ + mul(b.0, a.1) / gcd_;
    let denominator = mul(a.1, b.1) / gcd_;
    let regcd = gcd(numerator, denominator);

    return (numerator/regcd, denominator/regcd);
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let dice_count = input.trim().parse::<usize>().unwrap();

    let mut result = (0, 1);

    for _ in 0..dice_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap());

        let divisor = iter.next().unwrap();
        let sum = iter.next().unwrap();
        let fraction = (sum, divisor);

        result = fraction_add(result, fraction);
    }

    // println!("{:?}", result);
    let mut result = mul(result.0, invmod(result.1, DIVISOR));
    if result < 0 {
        result += DIVISOR;
    }
    println!("{}", result);
}
