use std::io::stdin;
use std::cmp::{ min, max };

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let node_count = iter.next().unwrap();
    let max_distance = iter.next().unwrap();
    let edge_count = iter.next().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let node_number = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut weights = vec![vec![usize::MAX; node_count]; node_count];
    for i in 0..node_count {
        weights[i][i] = 0;
    }

    for _ in 0..edge_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());

        let start = iter.next().unwrap() - 1;
        let end = iter.next().unwrap() - 1;
        let weight = iter.next().unwrap();

        weights[start][end] = weight;
        weights[end][start] = weight;
    }

    for i in 0..node_count {
        for j in 0..node_count {
            if weights[i][j] == usize::MAX {
                continue;
            }

            for k in 0..node_count {
                if weights[k][i] == usize::MAX {
                    continue;
                }

                let distance = weights[k][i] + weights[i][j];
                weights[k][j] = min(weights[k][j], distance);
            }
        }
    }

    let mut result = 0;
    for i in 0..node_count {
        let mut now_items = 0;
        for j in 0..node_count {
            if weights[i][j] <= max_distance {
                now_items += node_number[j];
            }
        }

        result = max(result, now_items);
    }

    println!("{}", result);
}
