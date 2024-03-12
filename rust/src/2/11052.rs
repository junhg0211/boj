use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let cards = input.trim().parse::<usize>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut prices = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    prices.insert(0, 0);

    let mut dp = prices.clone();
    for i in 1..=cards {
        for j in 0..=i / 2 {
            let price = dp[i - j] + dp[j];
            if price > dp[i] {
                dp[i] = price;
            }
        }
    }

    println!("{}", dp[cards]);
}
