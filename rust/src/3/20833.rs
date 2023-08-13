use std::io::stdin;

fn main() {
    let number = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    let mut result = 0;
    for i in 1..=number {
        result += i.pow(3);
    }

    println!("{}", result);
}
