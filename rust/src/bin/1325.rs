use std::io::stdin;
use std::collections::{ HashMap, HashSet, VecDeque };

fn main() {
    // --- get connections
    let (node_count, connection_count) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input.trim().split_whitespace().map(|x| x.parse::<u32>().unwrap());

        (
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    };

    let mut connections: HashMap<u32, HashSet<u32>> = HashMap::new();

    for _ in 0..connection_count {
        let (a, b) = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let mut iter = input.trim().split_whitespace().map(|x| x.parse::<u32>().unwrap());

            (
                iter.next().unwrap(),
                iter.next().unwrap(),
            )
        };

        if !connections.contains_key(&b) {
            connections.insert(b, HashSet::new());
        }

        connections.get_mut(&b).unwrap().insert(a);
    }

    // --- calculate which is greatest
    let mut max_infection = 0;
    let mut max_infectors = Vec::new();
    for i in 1..=node_count {
        let mut stack = VecDeque::from(vec![i]);
        let mut infectees = HashSet::new();

        while !stack.is_empty() {
            let i = stack.pop_front().unwrap();
            infectees.insert(i);

            if !connections.contains_key(&i) {
                continue;
            }

            for j in connections.get(&i).unwrap() {
                if infectees.contains(&j) {
                    continue;
                }

                stack.push_back(*j);
            }
        }

        if infectees.len() > max_infection {
            max_infectors.clear();
            max_infection = infectees.len();
        }

        if infectees.len() == max_infection {
            max_infectors.push(i);
        }
    }

    max_infectors.sort();

    for infector in max_infectors.iter() {
        print!("{} ", infector);
    }
}
