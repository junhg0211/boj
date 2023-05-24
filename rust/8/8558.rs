use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let until = input.trim().parse::<i32>().unwrap();

    if until >= 5 {
        println!("0");
        return;
    }

    let mut result = 1;
    for i in 2..(until+1) {
        result *= i;
        result %= 10;
    }

    println!("{}", result);
}
