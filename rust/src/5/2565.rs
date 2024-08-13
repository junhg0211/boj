use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<usize>().unwrap();

    let mut connections = Vec::new();
    for _ in 0..count {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        connections.push((a, b));
    }

    connections.sort();
    let mut dp = vec![1; count];
    let mut real_max = 1;
    for i in 0..count {
        let mut max = 1;
        for j in 0..i {
            if connections[j].1 < connections[i].1 {
                max = max.max(dp[j] + 1);
            }
        }
        dp[i] = max;
        real_max = max.max(real_max);
    }

    println!("{}", count - real_max);
}
