use std::io::stdin;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    if a < b {
        return gcd(b, a);
    }

    while b > 0 {
        (a, b) = (b, a%b);
    }

    return a;
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let number_count = input.trim().parse::<usize>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    numbers.sort();

    if number_count == 1 {
        println!("{}", numbers[0]);
        return;
    }

    let mut result = numbers[1] - numbers[0];
    for i in 1..number_count-1 {
        let diff = numbers[i+1] - numbers[i];
        result = gcd(result, diff);
    }

    println!("{}", result);
}
