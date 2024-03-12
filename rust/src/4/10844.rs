use std::io::stdin;

const DIVISOR: u32 = 1000000000;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<u32>().unwrap();

    let mut dp = [1; 10];
    dp[0] = 0;
    for _ in 1..count {
        let mut new_dp = [0; 10];
        for i in 0..10 {
            if i > 0 {
                new_dp[i] += dp[i - 1];
            }
            if i < 9 {
                new_dp[i] += dp[i + 1];
            }

            new_dp[i] %= DIVISOR;
        }

        dp = new_dp;
    }

    let mut result = 0;
    for i in 0..10 {
        result += dp[i];
        result %= DIVISOR;
    }

    println!("{}", result);
}
