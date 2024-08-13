use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let count = iter.next().unwrap();
    let max_weight = iter.next().unwrap();

    let mut things = Vec::new();
    for _ in 0..count {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());
        let weight = iter.next().unwrap();
        let value = iter.next().unwrap();
        things.push((weight, value));
    }

    let mut dp = vec![vec![0; max_weight + 1]; count + 1];
    for i in 1..=count {
        let weight = things[i - 1].0;
        let value = things[i - 1].1;
        for j in 1..=max_weight {
            let mut max = 0;

            if j >= things[i - 1].0 {
                max = max.max(dp[i - 1][j - weight] + value);
            }
            max = max.max(dp[i - 1][j]).max(dp[i][j - 1]);

            dp[i][j] = max;
        }
    }

    println!("{}", dp[count][max_weight]);
}
