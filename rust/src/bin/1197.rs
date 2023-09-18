use std::io::stdin;
use std::collections::{ BinaryHeap, HashMap };

fn main() {
    let (node_count, edge_count) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());

        (
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    };

    let mut edges = BinaryHeap::new();
    for _ in 0..edge_count {
        let (a, b, weight) = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let mut iter = input
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap());

            (
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            )
        };

        edges.push((-weight, a, b));
    }

    let mut connecteds: HashMap<i32, i32> = HashMap::new();
    let mut total_weight = 0;
    while connecteds.len() < node_count-1 {
        let (weight, a, b) = edges.pop().unwrap();

        let a_root = get_root(&a, &connecteds);
        let b_root = get_root(&b, &connecteds);

        println!("  {} {} {}: {} {} {:?}", a, b, weight, a_root, b_root, connecteds);

        if a_root == b_root {
            continue;
        }

        if connecteds.contains_key(&a) {
            connecteds.insert(b, a);
        } else {
            connecteds.insert(a, b);
        }
        total_weight -= weight;
    }

    println!("{}", total_weight);
}

fn get_root(node: &i32, connecteds: &HashMap<i32, i32>) -> i32 {
    if !connecteds.contains_key(&node) {
        return *node;
    }

    return get_root(connecteds.get(&node).unwrap(), connecteds);
}
