use std::io::stdin;

fn main() {
    let mut number = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<f64>().unwrap()
    };

    let mut log = 0;
    while number > 10.0 {
        number /= 10.0;
        number = number.round();

        log += 1;
    }

    println!("{}", number * 10.0f64.powi(log));
}
