use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();

    for i in 0..(input.len() / 2 + 1) {
        if input.chars().nth(input.len()-i-1).unwrap() != input.chars().nth(i).unwrap() {
            println!("0");
            return;
        }
    }

    println!("1");
}
