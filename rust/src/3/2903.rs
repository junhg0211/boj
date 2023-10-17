use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let iteration_count = input.trim().parse::<u32>().unwrap();

    let mut side = 2;
    for _ in 0..iteration_count {
        side = side * 2 - 1;
    }

    println!("{}", side * side);
}
