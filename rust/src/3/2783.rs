use std::io::stdin;

fn get_price() -> f64 {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let numbers: Vec<f64> = input.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    1000.0 / numbers[1] * numbers[0]
}

fn main() {
    // get current price
    let mut result = get_price();

    // get count of conbini
    let count: u32 = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse().unwrap()
    };

    // calculate min price
    for _ in 0..count {
        let price = get_price();

        if price < result {
            result = price;
        }
    }

    println!("{:.2}", result);
}
