use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let a = input.trim().chars().collect::<Vec<_>>();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let b = input.trim().chars().collect::<Vec<_>>();

    let mut dp = vec![vec![String::new(); b.len()]; a.len()];
    for i in 0..a.len() {
        for j in 0..b.len() {
            if a[i] == b[j] {
                if i > 0 {
                    dp[i][j] = dp[i - 1][j].clone();
                }
                dp[i][j].push(a[i]);
                continue;
            }

            if i > 0 {
                dp[i][j] = dp[i - 1][j].clone();
            }

            if j > 0 && dp[i][j].len() <= dp[i][j - 1].len() {
                dp[i][j] = dp[i][j - 1].clone();
                continue;
            }
        }
    }

    /*
    for row in dp.iter() {
        println!("{:?}", row);
    }
    */

    let string = &dp[a.len() - 1][b.len() - 1];
    println!("{}\n{}", string.len(), string);
}
