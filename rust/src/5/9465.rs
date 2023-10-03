use std::io::stdin;
use std::cmp::max;

fn tick() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let width = input.trim().parse::<usize>().unwrap();

    let mut stickers = Vec::new();
    for _ in 0..2 {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let row = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        stickers.push(row);
    }

    let mut dp = (0, stickers[0][0], stickers[1][0]);
    // not, up, down
    for i in 1..width {
        let not = max(dp.1, dp.2);
        let up = max(dp.0, dp.2) + stickers[0][i];
        let down = max(dp.0, dp.1) + stickers[1][i];

        dp = (not, up, down);
    }

    let result = max(max(dp.0, dp.1), dp.2);
    println!("{}", result);
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let testcase_count = input.trim().parse::<usize>().unwrap();

    for _ in 0..testcase_count {
        tick();
    }
}
