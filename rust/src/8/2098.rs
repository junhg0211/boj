use std::cmp::min;
use std::io::stdin;

fn tsp(
    here: usize,
    starting: usize,
    visited: usize,
    weights: &Vec<Vec<u32>>,
    dp: &mut Vec<Vec<u32>>,
) -> u32 {
    if dp[here][visited] != 0 {
        return dp[here][visited];
    }

    if visited == (1 << weights.len()) - 1 {
        if weights[here][starting] == 0 {
            return u32::MAX;
        }

        return weights[here][starting];
    }

    let mut result = u32::MAX;
    for i in 0..weights.len() {
        if weights[here][i] == 0 {
            continue;
        }
        if visited & (1 << i) != 0 {
            continue;
        }

        let new_visited = visited | (1 << i);
        let extra_distance = tsp(i, starting, new_visited, weights, dp);

        if extra_distance == u32::MAX {
            continue;
        }

        let distance = weights[here][i] + extra_distance;
        result = min(result, distance);
    }

    dp[here][visited] = result;
    return result;
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let city_count = input.trim().parse::<usize>().unwrap();

    let mut weights = Vec::new();
    for _ in 0..city_count {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let row = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        weights.push(row);
    }

    let mut dp = vec![vec![0; 1 << weights.len()]; weights.len()];
    let distance = tsp(0, 0, 1, &weights, &mut dp);

    println!("{}", distance);
}
