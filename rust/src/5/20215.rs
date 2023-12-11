use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap());

    let width = iter.next().unwrap();
    let height = iter.next().unwrap();

    let rectangle_length = width + height;
    let diagonal_length = (width * width + height * height).sqrt();

    println!("{}", rectangle_length - diagonal_length);
}
