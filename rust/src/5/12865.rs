use std::io::stdin;
use std::cmp::max;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let thing_count = iter.next().unwrap();
    let max_weight = iter.next().unwrap();

    let mut things = Vec::new();
    for _ in 0..thing_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());

        let weight = iter.next().unwrap();
        let value = iter.next().unwrap();

        things.push((weight, value));
    }

    let mut dp = vec![vec![0; max_weight+1]; thing_count+1];
    for i in 1..=thing_count {
        for j in 1..=max_weight {
            let (weight, value) = things[i-1];

            if weight > j {
                dp[i][j] = dp[i-1][j];
            } else {
                dp[i][j] = max(dp[i-1][j], dp[i-1][j-weight] + value);
            }
        }
    }

    println!("{}", dp[thing_count][max_weight]);
}
