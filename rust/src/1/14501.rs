use std::cmp::max;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let size = input.trim().parse::<usize>().unwrap();

    let mut dp = vec![vec![0; size]; size];
    for i in 0..size {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());

        let days = iter.next().unwrap();
        let cost = iter.next().unwrap();

        let from_day = i + days - 1;
        if from_day >= size {
            continue;
        }

        let wow = max(if i > 0 { dp[i][i - 1] } else { 0 } + cost, dp[i][from_day]);
        for k in i..size {
            for j in from_day..size {
                dp[k][j] = max(dp[k][j], wow);
            }
        }
    }

    /*
    for row in &dp {
        println!("{:?}", row);
    }
    */

    println!("{}", dp[size - 1][size - 1]);
}
