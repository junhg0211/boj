use std::io;
use std::f64::consts::PI;

fn main() {
    let mut input = String::new();

    // get width
    io::stdin().read_line(&mut input).unwrap();
    let width = input.trim().parse::<u32>().unwrap();

    // get radius
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let radius = input.trim().parse::<u32>().unwrap();

    // calculate and print
    let result = 2.0 * PI * radius as f64 + width as f64 * 2.0;
    println!("{}", result);
}
