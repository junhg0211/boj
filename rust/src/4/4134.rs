use std::io::stdin;

fn sqrt(number: u32) -> u32 {
    return (number as f64).sqrt() as u32;
}

fn is_prime(number: u32) -> bool {
    for i in 2..=sqrt(number) {
        if number % i == 0 {
            return false;
        }
    }

    return true;
}

fn tick() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut number = input.trim().parse::<u32>().unwrap();

    if number < 2 {
        println!("2");
        return;
    }

    loop {
        if is_prime(number) {
            println!("{}", number);
            return;
        }

        number += 1;
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
