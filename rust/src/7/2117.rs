use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<u32>().unwrap() - 1;
    println!("{}", count * count / 4);
}
