use std::io::stdin;
use std::cmp::max;

fn main() {
    // get strings
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let a_string = input.trim().chars().collect::<Vec<char>>();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let b_string = input.trim().chars().collect::<Vec<char>>();

    // find lcs
    let mut dp = vec![vec![0; b_string.len()+1]; a_string.len()+1];

    for i in 1..=a_string.len() {
        let a_char = a_string[i-1];

        for j in 1..=b_string.len() {
            let b_char = b_string[j-1];

            if a_char != b_char {
                dp[i][j] = max(dp[i-1][j], dp[i][j-1]);
            } else {
                dp[i][j] = dp[i-1][j-1] + 1;
            };
        }
    }

    println!("{}", dp[a_string.len()][b_string.len()]);
}
