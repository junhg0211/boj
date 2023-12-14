use std::io::stdin;
use std::collections::HashSet;
use std::cmp::min;

fn generate_hash(waters: (u64, u64, u64), volumes: (u64, u64, u64)) -> u64 {
    (waters.0 * volumes.1 + waters.1) * volumes.2 + waters.2
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut volumes = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap());
    let volumes = (
        volumes.next().unwrap(),
        volumes.next().unwrap(),
        volumes.next().unwrap(),
    );

    let mut stack = Vec::new();
    let mut beens = HashSet::new();
    stack.push((0, 0, volumes.2));
    beens.insert(generate_hash((0, 0, volumes.2), volumes));
    let mut possibles = HashSet::new();
    while let Some(waters) = stack.pop() {
        if waters.0 == 0 {
            possibles.insert(waters.2);
        }

        // A
        if waters.0 > 0 {
            // L
            let amount = min(waters.0, volumes.1 - waters.1);
            let state = (waters.0 - amount, waters.1 + amount, waters.2);
            let hash = generate_hash(state, volumes);
            if !beens.contains(&hash) {
                stack.push(state);
                beens.insert(hash);
            }

            // R
            let amount = min(waters.0, volumes.2 - waters.2);
            let state = (waters.0 - amount, waters.1, waters.2 + amount);
            let hash = generate_hash(state, volumes);
            if !beens.contains(&hash) {
                stack.push(state);
                beens.insert(hash);
            }
        }

        // B
        if waters.1 > 0 {
            // L
            let amount = min(waters.1, volumes.0 - waters.0);
            let state = (waters.0 + amount, waters.1 - amount, waters.2);
            let hash = generate_hash(state, volumes);
            if !beens.contains(&hash) {
                stack.push(state);
                beens.insert(hash);
            }

            // R
            let amount = min(waters.1, volumes.2 - waters.2);
            let state = (waters.0, waters.1 - amount, waters.2 + amount);
            let hash = generate_hash(state, volumes);
            if !beens.contains(&hash) {
                stack.push(state);
                beens.insert(hash);
            }
        }

        // C
        if waters.2 > 0 {
            // L
            let amount = min(waters.2, volumes.0 - waters.0);
            let state = (waters.0 + amount, waters.1, waters.2 - amount);
            let hash = generate_hash(state, volumes);
            if !beens.contains(&hash) {
                stack.push(state);
                beens.insert(hash);
            }

            // R
            let amount = min(waters.2, volumes.1 - waters.1);
            let state = (waters.0, waters.1 + amount, waters.2 - amount);
            let hash = generate_hash(state, volumes);
            if !beens.contains(&hash) {
                stack.push(state);
                beens.insert(hash);
            }
        }
    }

    let mut result = possibles.drain().collect::<Vec<_>>();
    result.sort();
    for number in result {
        print!("{} ", number);
    }
    println!();
}
