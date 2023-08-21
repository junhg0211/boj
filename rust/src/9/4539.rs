use std::io::stdin;

fn main() {
    let count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    for _ in 0..count {
        let mut number = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            input.trim().parse::<f64>().unwrap()
        };

        let log = number.log10().floor() as u32;

        for _ in 0..log {
            number /= 10.0;
            number = number.round();
        }

        println!("{}", (number * 10.0f64.powf(log as f64)) as u32);
    }
}
