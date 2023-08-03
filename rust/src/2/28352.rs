use std::io::stdin;

fn main() {
    let number = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    let mut result = 6;

    for i in 11..=number {
        result *= i;
    };

    println!("{}", result);
}
