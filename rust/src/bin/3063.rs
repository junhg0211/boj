use std::io::stdin;
use std::cmp::{min, max};

fn tick() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());

    let x1 = iter.next().unwrap();
    let y1 = iter.next().unwrap();
    let x2 = iter.next().unwrap();
    let y2 = iter.next().unwrap();
    let x3 = iter.next().unwrap();
    let y3 = iter.next().unwrap();
    let x4 = iter.next().unwrap();
    let y4 = iter.next().unwrap();

    let original_area = (x2-x1) * (y2-y1);
    let xs = min(x2, x4) - max(x1, x3);
    let ys = min(y2, y4) - max(y1, y3);
    let duplicated = xs * ys;

    if duplicated > original_area {
        println!("0");
    } else {
        println!("{}", original_area - duplicated);
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