use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u64>().unwrap();

    println!("{}", n * (n*n - 3*n + 2) / 6);
    println!("3");
}
