use std::io::stdin;

fn tick(input: &mut String) {
    // get cents
    input.clear();
    stdin().read_line(input).unwrap();

    // calculate
    let mut cents: u32 = input.trim().parse().unwrap();
    let quarters: u32 = cents / 25;
    cents %= 25;
    let dimes: u32 = cents / 10;
    cents %= 10;
    let nickels: u32 = cents / 5;
    cents %= 5;

    println!("{} {} {} {}", quarters, dimes, nickels, cents);
}

fn main() {
    // get count
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let count: u32 = input.trim().parse().unwrap();

    // iterate
    for _ in 0..count {
        tick(&mut input);
    }
}
