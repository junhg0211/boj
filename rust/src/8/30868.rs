use std::io::stdin;

fn tick() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let number = input.trim().parse::<u32>().unwrap();

    for _ in 0..number/5 {
        print!("++++ ");
    }

    for _ in 0..number%5 {
        print!("|");
    }
    println!();
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let testcase_count = input.trim().parse::<usize>().unwrap();

    for _ in 0..testcase_count {
        tick();
    }
}
