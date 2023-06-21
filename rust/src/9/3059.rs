use std::io::stdin;

fn tick(input: &mut String) {
    // get input
    input.clear();
    stdin().read_line(input).unwrap();
    let input = input.trim();

    let mut result = 0;

    for c in 'A'..='Z' {
        if !input.contains(c) {
            result += c as u32;
        }
    }

    println!("{}", result);
}

fn main() {
    // get count
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<u32>().unwrap();

    for _ in 0..count {
        tick(&mut input);
    }
}
