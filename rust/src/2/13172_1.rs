use std::io::stdin;

const DIVISOR: i64 = 1000000007;

fn mul(a: i64, b: i64) -> i64 {
    let result = (a % DIVISOR) * (b % DIVISOR) % DIVISOR;
    if result < 0 {
        return result + DIVISOR;
    }
    return result;
}

/// ta \equiv 1 \mod s
fn eea(t: i64, s: i64) -> (i64, i64) {
    // sa'' + ta' = 1
    // t(a' + qa'') + ra'' = 1

    let (q, r) = (s/t, s%t);
    // ta'' + ra' = 1

    if r == 0 {
        return (1, 0);
        // a, a'
    }

    let (ap, app) = eea(r, t);
    let a = (app - q*ap) % s;
    return (a, ap);
}

fn invmod(of: i64, when: i64) -> i64 {
    return eea(of, when).0;
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
