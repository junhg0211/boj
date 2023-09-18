use std::io::stdin;
use std::cmp::min;

fn main() {
    let number = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<usize>().unwrap()
    };

    let mut dp = vec![0, 1];
    for i in 2..=number {
        let mut count = u32::MAX;
        let mut j = 1;
        while j*j <= i {
            count = min(count, dp[i - j*j] + 1);
            j += 1;
        }
        dp.push(count);
    }

    println!("{:?}", dp[number]);
}
