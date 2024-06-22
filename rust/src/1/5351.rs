use std::io::stdin;

fn tick() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());

    let mut dp = (0, 0);
    // possible, impossible
    for number in iter {
        dp = (if dp.0 > dp.1 { dp.0 } else { dp.1 }, dp.0 + number);
        // println!("{:?}", dp);
    }

    println!("{}", if dp.0 > dp.1 { dp.0 } else { dp.1 });
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let testcase_count = input.trim().parse::<usize>().unwrap();

    for _ in 0..testcase_count {
        tick();
    }
}
