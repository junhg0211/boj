use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let s = input.trim().parse::<f64>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let a = input.trim().parse::<f64>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let b = input.trim().parse::<f64>().unwrap();

    let extra_count = ((s-a)/b).ceil();
    let extra_count = if extra_count <= 0.0 {
        0.0
    } else {
        extra_count
    };

    println!("{}", 250.0 + 100.0 * extra_count);
}
