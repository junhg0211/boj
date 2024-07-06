use std::{collections::HashMap, io::stdin};

const DIVISOR: usize = 1000000000;

fn f(n: usize, k: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if k == 1 {
        return 1;
    }
    if n == 0 {
        return 1;
    }

    if cache.contains_key(&(n, k)) {
        return *cache.get(&(n, k)).unwrap();
    }

    let mut result = 0;
    for i in 0..=n {
        result = (result + f(n - i, k - 1, cache)) % DIVISOR;
    }

    cache.insert((n, k), result);

    result
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n = iter.next().unwrap();
    let k = iter.next().unwrap();

    let mut cache = HashMap::<(usize, usize), usize>::new();
    println!("{}", f(n, k, &mut cache));
}
