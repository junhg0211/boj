use std::cmp::{max, min};
use std::io::stdin;

fn gcd(mut a: u32, mut b: u32) -> u32 {
    if a < b {
        return gcd(b, a);
    }

    while b > 0 {
        (a, b) = (b, a % b);
    }

    return a;
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let tree_count = input.trim().parse::<usize>().unwrap();

    let mut tree_positions = Vec::new();
    let mut min_position = u32::MAX;
    let mut max_position = u32::MIN;
    for _ in 0..tree_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let tree_position = input.trim().parse::<u32>().unwrap();
        tree_positions.push(tree_position);

        min_position = min(min_position, tree_position);
        max_position = max(max_position, tree_position);
    }

    let mut diffs = Vec::new();
    for i in 0..tree_count - 1 {
        let diff = tree_positions[i + 1] - tree_positions[i];
        diffs.push(diff);
    }

    let mut result = diffs.pop().unwrap();
    while let Some(diff) = diffs.pop() {
        result = gcd(result, diff);
    }

    println!(
        "{}",
        (max_position - min_position) / result - tree_count as u32 + 1
    );
}
