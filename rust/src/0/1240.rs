use std::io::stdin;
use std::collections::{ HashMap, VecDeque, HashSet };

fn main() {
    // --- get counts
    let (node_count, tuple_count) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap());

        (
            iter.next().unwrap(),
            iter.next().unwrap()
        )
    };

    // --- get connections
    let mut connections = HashMap::new();
    for _ in 0..node_count-1 {
        let (a, b, distance) = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let mut iter = input
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap());

            (
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap()
            )
        };

        if !connections.contains_key(&a) {
            connections.insert(a, Vec::new());
        }
        if !connections.contains_key(&b) {
            connections.insert(b, Vec::new());
        }

        connections.get_mut(&a).unwrap().push((b, distance));
        connections.get_mut(&b).unwrap().push((a, distance));
    }

    // --- get tuples and calculate result
    for _ in 0..tuple_count {
        let (a, b) = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let mut iter = input
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap());

            (
                iter.next().unwrap(),
                iter.next().unwrap()
            )
        };

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((a, 0));
        visited.insert(a);
        while !queue.is_empty() {
            let now = queue.pop_front().unwrap();

            if now.0 == b {
                println!("{}", now.1);
                break;
            }

            for (target, distance) in connections.get(&now.0).unwrap() {
                if visited.contains(target) {
                    continue;
                }

                queue.push_back((*target, now.1 + distance));
                visited.insert(*target);
            }
        }
    }
}
