use std::io::stdin;
use std::collections::HashSet;
use std::cmp::min;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<usize>().unwrap();

    let mut quarters = HashSet::new();
    let mut i = 1;
    loop {
        let number = i * (i*i + 3*i + 2) / 6;
        if number > 300000 {
            break;
        }
        quarters.insert(number);
        i += 1;
    }

    let mut dp = vec![u32::MAX; count+1];
    for i in 1..=count {
        if quarters.contains(&i) {
            dp[i] = 1;
            continue;
        }

        for quarter in &quarters {
            if quarter > &i {
                continue;
            }

            dp[i] = min(dp[i], 1 + dp[i-quarter]);
        }

        // println!("{} {}", i, dp[i]);
    }

    println!("{}", dp[count]);
}
