use std::io::stdin;
use std::cmp::min;

fn add(a: u32, b: u32) -> u32 {
    if a >= u32::MAX - b {
        return u32::MAX;
    }

    a + b
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let color_count = input.trim().parse::<usize>().unwrap();

    let mut dp = vec![vec![u32::MAX, u32::MAX, u32::MAX]; 3];
    for i in 0..color_count {
        // println!("{:?}", dp);

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let costs = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        if i == 0 {
            for j in 0..3 {
                dp[j][j] = costs[j];
            }
            continue;
        }

        for j in 0..3 {
            (dp[j][0], dp[j][1], dp[j][2]) = (
                add(costs[0], min(dp[j][1], dp[j][2])),
                add(costs[1], min(dp[j][0], dp[j][2])),
                add(costs[2], min(dp[j][0], dp[j][1]))
            );
        }
    }

    // println!("{:?}", dp);

    let mut result = dp[0][1];
    result = min(result, dp[0][2]);
    result = min(result, dp[1][0]);
    result = min(result, dp[1][2]);
    result = min(result, dp[2][0]);
    result = min(result, dp[2][1]);

    println!("{}", result);
}
