use std::io::stdin;

fn tick() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap());
    let start = iter.next().unwrap();
    let end = iter.next().unwrap();

    let diff = end - start;
    let result = (4.0 * diff - 1.0).sqrt().ceil() - 1.0;
    println!("{}", result as u32);
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let testcase_count = input.trim().parse::<usize>().unwrap();

    for _ in 0..testcase_count {
        tick();
    }
}
