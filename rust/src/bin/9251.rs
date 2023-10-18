use std::io::stdin;
use std::cmp::max;

fn main() {
    // -- get strings
    let mut a_string = String::new();
    stdin().read_line(&mut a_string).unwrap();
    a_string = a_string.trim().to_string();

    let mut b_string = String::new();
    stdin().read_line(&mut b_string).unwrap();
    b_string = b_string.trim().to_string();

    // -- longest common string algorithm
    let mut dp = vec![vec![0; b_string.len() + 1]; a_string.len() + 1];
    for i in 1..=a_string.len() {
        let a_char = a_string.chars().nth(i-1).unwrap();

        for j in 1..=b_string.len() {
            let b_char = b_string.chars().nth(j-1).unwrap();

            if a_char == b_char {
                dp[i][j] = dp[i-1][j-1] + 1;
            } else {
                dp[i][j] = max(dp[i-1][j], dp[i][j-1]);
            }
        }
    }

    // -- generate string by the lcs dp
    let mut stack = Vec::new();
    let mut y = a_string.len();
    let mut x = b_string.len();
    while x > 0 && y > 0 {
        if dp[y-1][x] == dp[y][x] {
            y -= 1;
        } else if dp[y][x-1] == dp[y][x] {
            x -= 1;
        } else {
            stack.push(a_string.chars().nth(y-1).unwrap());
            x -= 1;
            y -= 1;
        }
    }

    // -- print result
    loop {
        match stack.pop() {
            Some(thing) => print!("{}", thing),
            None => break,
        }
    }
    println!();
}
