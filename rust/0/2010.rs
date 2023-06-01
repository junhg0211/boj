use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let count = input.trim().parse::<u32>().unwrap();

    let mut result = 1;

    for _ in 0..count {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        result += input.trim().parse::<u32>().unwrap() - 1;
    }

    println!("{}", result);
}
