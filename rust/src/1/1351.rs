use std::io::stdin;
use std::collections::HashMap;

fn f(i: u64, p: u64, q: u64, dp: &mut HashMap<u64, u64>) -> u64 {
    if i == 0 {
        return 1;
    }
    if dp.contains_key(&i) {
        return *dp.get(&i).unwrap();
    }

    let value = f(i/p, p, q, dp) + f(i/q, p, q, dp);
    dp.insert(i, value);
    return value;
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap());
    let n = iter.next().unwrap();
    let p = iter.next().unwrap();
    let q = iter.next().unwrap();

    let mut dp = HashMap::new();
    let result = f(n, p, q, &mut dp);
    println!("{}", result);
}
