use std::cmp::max;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<usize>().unwrap();

    let mut dp = (0, 0, 0);
    for _ in 0..count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let size = input.trim().parse::<u32>().unwrap();

        dp = (dp.1 + size, dp.2 + size, max(max(dp.0, dp.1), dp.2));
    }

    println!("{}", max(max(dp.0, dp.1), dp.2));
}
