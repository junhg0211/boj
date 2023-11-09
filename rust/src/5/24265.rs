use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let number = input.trim().parse::<u64>().unwrap();

    println!("{}", number * (number - 1) / 2);
    println!("2");
}
