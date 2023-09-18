use std::io::stdin;
use std::cmp::max;

fn main() {
    let count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<usize>().unwrap()
    };

    let mut costs = Vec::new();
    for _ in 0..count {
        let cost = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            input.trim().parse::<u32>().unwrap()
        };

        costs.push(cost);
    }

    if count == 1 {
        println!("{}", costs[0]);
        return;
    }

    let mut dp = vec![(costs[0], costs[0]), (costs[1], costs[0] + costs[1])];
    // double, single
    for i in 2..count {
        // double
        let double = costs[i] + max(dp[i-2].0, dp[i-2].1);
        let single = costs[i] + dp[i-1].0;
        dp.push((double, single));
    }

    println!("{}", max(dp[count-1].0, dp[count-1].1));
}
