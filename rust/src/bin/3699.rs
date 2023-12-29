use std::io::stdin;
use std::cmp::{ max, min };

fn tick() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let height = iter.next().unwrap();
    let width = iter.next().unwrap();

    let mut positions = vec![(0, 0); height * width];
    let mut car_count = 0;
    for i in 0..height {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let orders = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        for j in 0..width {
            if orders[j] != -1 {
                positions[orders[j] as usize] = (i, j);
                car_count = max(orders[j], car_count);
            }
        }
    }
    let car_count = car_count as usize;

    let mut result = 0;
    for i in 0..car_count {
        let a = positions[i];
        let b = positions[i+1];

        let inside = if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 };
        let outside = width - inside;
        let dx = min(inside, outside);

        let dy = b.0 * 2;

        result += dx * 5 + dy * 10;
    }

    println!("{}", result);
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let testcase_count = input.trim().parse::<usize>().unwrap();

    for _ in 0..testcase_count {
        tick();
    }
}
