use std::io::stdin;

fn diff(a: u32, b: u32) -> u32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn cost(i: usize, j: usize, numbers: &Vec<u32>) -> u32 {
    (j - i) as u32 * (1 + diff(numbers[j], numbers[i]))
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let number_count = input.trim().parse::<usize>().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut dp = vec![vec![0; number_count]; number_count];
    for i in 0..number_count {
        let cost_to_i = if i == 0 {
            0
        } else {
            (0..i).map(|j| dp[j][i]).min().unwrap()
        };

        for j in i + 1..number_count {
            dp[i][j] = u32::max(cost_to_i, cost(i, j, &numbers));
        }
    }

    // println!("{:?}", dp);
    println!(
        "{}",
        (0..number_count - 1)
            .map(|i| dp[i][number_count - 1])
            .min()
            .unwrap()
    );
}
