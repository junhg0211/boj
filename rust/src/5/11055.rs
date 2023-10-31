use std::io::stdin;
use std::cmp::max;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let number_count = input.trim().parse::<usize>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut result = 0;
    let mut dp = Vec::new();
    for i in 0..number_count {
        let mut record = numbers[i];
        for j in 0..i {
            if numbers[j] >= numbers[i] {
                continue;
            }

            record = max(dp[j] + numbers[i], record);
        }

        dp.push(record);

        result = max(record, result);
    }

    println!("{}", result);
    // println!("{:?}", dp);
}
