use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let x = numbers[0];
    let l = numbers[1];
    let r = numbers[2];

    let mut distance = (l-x).abs();
    let mut result = l;

    for i in l..(r+1) {
        if (i-x).abs() < distance {
            distance = (i-x).abs();
            result = i;
        }
    }

    println!("{}", result);
}
