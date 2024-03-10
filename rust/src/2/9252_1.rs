use std::cmp::max;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let string1 = input.trim().chars().collect::<Vec<_>>();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let string2 = input.trim().chars().collect::<Vec<_>>();

    let mut dp = vec![vec![0; string1.len() + 1]; string2.len() + 1];

    for i in 0..=string2.len() {
        for j in 0..=string1.len() {
            if i == 0 || j == 0 {
                dp[i][j] = 0;
                continue;
            }

            if string1[j - 1] == string2[i - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
                continue;
            }

            dp[i][j] = max(dp[i - 1][j], dp[i][j - 1]);
        }
    }

    let mut stack = Vec::new();
    let (mut i, mut j) = (string2.len(), string1.len());
    while dp[i][j] > 0 {
        if dp[i - 1][j] == dp[i][j] {
            i -= 1;
            continue;
        }

        if dp[i][j - 1] == dp[i][j] {
            j -= 1;
            continue;
        }

        i -= 1;
        j -= 1;
        stack.push(string1[j]);
    }

    println!("{}", stack.len());
    while let Some(letter) = stack.pop() {
        print!("{}", letter);
    }
    println!();
}
