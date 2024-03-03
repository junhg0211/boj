use std::collections::HashSet;
use std::io::stdin;

fn log10(number: u64) -> u32 {
    let mut result = 0;
    let mut tmp = 1;

    while number >= tmp {
        tmp *= 10;
        result += 1;
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

    let n = iter.next().unwrap();
    let k = iter.next().unwrap();

    let mut remainders = HashSet::new();
    let mut tries = 1;
    let mut now = n;
    loop {
        now %= k;
        // println!("{}", now);

        if now == 0 {
            println!("{}", tries);
            return;
        }

        if remainders.contains(&now) {
            println!("-1");
            return;
        }

        remainders.insert(now);

        tries += 1;

        now *= u64::pow(10, log10(n));
        now += n;
    }
}
