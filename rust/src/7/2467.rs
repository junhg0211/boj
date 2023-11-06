use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<usize>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut i = 0;
    let mut j = count-1;
    let mut min = i32::MAX;
    let mut result = (0, 0);
    while i < j {
        let left = numbers[i];
        let right = numbers[j];

        let this = left + right;
        if this.abs() < min.abs() {
            min = this;
            result = (left, right);
        }

        // println!("{} {} {} {:?}", left, right, min, result);

        if left.abs() > right.abs() {
            i += 1;
        } else {
            j -= 1;
        }
    }

    println!("{} {}", result.0, result.1);
}
