use std::{collections::HashSet, io::stdin};

fn tick() {
    stdin().read_line(&mut String::new()).unwrap();

    let mut a_set = HashSet::new();
    let mut b_set = HashSet::new();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    for number in iter {
        a_set.insert(number);
    }

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    for number in iter {
        b_set.insert(number);
    }

    let union_count = a_set.union(&b_set).count() as f64;
    let intersection_count = a_set.intersection(&b_set).count() as f64;

    let j = intersection_count / union_count;

    if j > 0.5 {
        println!("1");
    } else {
        println!("0");
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let testcase_count = input.trim().parse::<usize>().unwrap();

    for _ in 0..testcase_count {
        tick();
    }
}
