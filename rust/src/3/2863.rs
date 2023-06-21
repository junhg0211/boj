use std::io::stdin;

fn main() {
    // get a, b, c, d
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();
    let numbers = input
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();
    let mut a = numbers[0];
    let mut b = numbers[1];

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let numbers = input
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();
    let mut c = numbers[0];
    let mut d = numbers[1];

    // get max number by iterate
    let mut max_count = 0;
    let mut max = a/c + b/d;
    for i in 1..4 {
        (a, b, c, d) = (c, a, d, b);
        let number = a/c + b/d;

        if number > max {
            max = number;
            max_count = i;
        }
    }

    // output
    println!("{}", max_count);
}
