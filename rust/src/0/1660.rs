use std::io::stdin;
use std::cmp::min;

fn get_m(n: u32) -> u32 {
    return n * (n*n + 3*n + 2) / 6;
}

fn get_n(m: u32) -> u32 {
    let mut result = 1;
    loop {
        if get_m(result) > m {
            return result;
        }

        result += 1;
    }
}

fn get_answer(mut count: u32) -> u32 {
    let mut result = 0;
    while count > 0 {
        let diff = get_m(get_n(count) - 1);
        count -= diff;
        result += 1;
    }
    return result;
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<usize>().unwrap();

    let mut dp = vec![0, 1];
    for i in 2..=count {
        let mut result = u32::MAX;
        for j in 1..=i/2 {
            result = min(result, get_answer(i as u32));
            result = min(result, dp[j] + dp[i-j]);
        }

        dp.push(result);
    }

    println!("{:?}", dp[count]);
}
