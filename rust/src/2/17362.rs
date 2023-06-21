use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let number = (input.trim().parse::<u32>().unwrap() - 1) % 8;

    println!("{}", match number {
        0 => 1,
        1 => 2,
        2 => 3,
        3 => 4,
        4 => 5,
        5 => 4,
        6 => 3,
        7 => 2,
        _ => 1,
    });
}
