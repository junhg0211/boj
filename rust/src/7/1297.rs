use std::io::stdin;

fn main() {
    // get numbers
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let numbers: Vec<f64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let diagonal = numbers[0];
    let height = numbers[1];
    let width = numbers[2];

    // calculate size
    let unit = diagonal / (width.powi(2) + height.powi(2)).sqrt();

    let height = height * unit;
    let width = width * unit;

    println!("{} {}", height as i32, width as i32);
}
