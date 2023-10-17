use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut number = input.trim().parse::<u32>().unwrap();

    let mut divisor = 2;
    while number > 1 {
        while number % divisor == 0 {
            println!("{}", divisor);
            number /= divisor;
        }

        divisor += 1;
    }
}
