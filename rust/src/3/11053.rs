use std::io::stdin;
use std::cmp::max;

fn main() {
    let count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<usize>().unwrap()
    };

    let numbers = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    };

    let mut dp: Vec<_> = Vec::new();
    let mut all_max = 0;
    for i in 0..count {
        let mut record = 0;
        for j in 0..i {
            if numbers[j] < numbers[i] {
                record = max(record, dp[j]+1);
            }
        }
        dp.push(record);

        all_max = max(all_max, record);
    }

    println!("{}", all_max + 1);
}
