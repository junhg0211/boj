use std::io::stdin;

const BASE: u32 = 1000000000;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let size = input.trim().parse::<usize>().unwrap();

    let mut dp = [(1, 0, 0, 0); 10];
    dp[0] = (0, 0, 0, 0);
    dp[9] = (0, 0, 1, 0);

    for _ in 1..size {
        let mut new_dp = [(0, 0, 0, 0); 10];

        for i in 0..10 {
            let nos;
            let zeros;
            let nines;
            let boths;

            if i == 0 {
                nos = 0;
                zeros = dp[1].0 + dp[1].1;
                nines = 0;
                boths = dp[1].2 + dp[1].3;
            } else if i == 9 {
                nos = 0;
                zeros = 0;
                nines = dp[8].0 + dp[8].2;
                boths = dp[8].1 + dp[8].3;
            } else {
                nos = dp[i - 1].0 + dp[i + 1].0;
                zeros = dp[i - 1].1 + dp[i + 1].1;
                nines = dp[i - 1].2 + dp[i + 1].2;
                boths = dp[i - 1].3 + dp[i + 1].3;
            }

            new_dp[i] = (nos % BASE, zeros % BASE, nines % BASE, boths % BASE);
        }

        dp = new_dp;
    }

    let mut total = 0;
    for i in 0..10 {
        // println!("{} {:?}", i, dp[i]);

        total += dp[i].3;
        total %= BASE;
    }

    println!("{}", total);
}
