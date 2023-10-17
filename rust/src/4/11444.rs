use std::io::stdin;
use std::collections::HashMap;

fn fib(n: u64, dp: &mut HashMap<u64, u64>) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return n;
    }
    if n == 2 {
        return 1;
    }

    if dp.contains_key(&n) {
        return *dp.get(&n).unwrap();
    }

    let result = if n % 2 == 0 {
        let n2 = fib(n/2, dp);
        let n2p1 = fib(n/2 + 1, dp);
        let n2m1 = fib(n/2 - 1, dp);

        n2 * n2p1 + n2m1 * n2
    } else {
        let np12 = fib((n + 1) / 2, dp);
        let nm12 = fib((n - 1) / 2, dp);

        np12 * np12 + nm12 * nm12
    } % 1000000007;

    dp.insert(n, result);
    return result;
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u64>().unwrap();

    let mut dp = HashMap::new();
    println!("{}", fib(n, &mut dp));
}
