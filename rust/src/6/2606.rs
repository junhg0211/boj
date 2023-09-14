use std::io::stdin;
use std::collections::{ HashMap, HashSet, VecDeque };

fn main() {
    // --- get counts
    let _computer_count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    let connection_count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    // --- get connections
    let mut connections = HashMap::new();
    for _ in 0..connection_count {
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

        if !connections.contains_key(&a) {
            connections.insert(a, Vec::new());
        }
        if !connections.contains_key(&b) {
            connections.insert(b, Vec::new());
        }

        connections.get_mut(&a).unwrap().push(b);
        connections.get_mut(&b).unwrap().push(a);
    }

    // --- get infecteds
    let mut infecteds = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(1);
    infecteds.insert(1);

    while !queue.is_empty() {
        let now = queue.pop_front().unwrap();

        if !connections.contains_key(&now) {
            continue;
        }

        for connection in connections[&now].iter() {
            if infecteds.contains(&connection) {
                continue;
            }

            infecteds.insert(*connection);
            queue.push_back(*connection);
        }
    }

    println!("{:?}", infecteds.len() - 1);
}
