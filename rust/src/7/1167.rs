use std::io::stdin;
use std::collections::{ HashMap, VecDeque, HashSet };

fn get_farest(from: i32, connections: &HashMap<i32, Vec<(i32, i32)>>) -> (i32, i32) {
    let mut queue = VecDeque::new();
    let mut been = HashSet::new();

    queue.push_back((from, 0));
    been.insert(from);

    let mut result = -1;
    let mut max_distance = -1;
    while !queue.is_empty() {
        let (now, distance) = queue.pop_front().unwrap();

        if distance > max_distance {
            max_distance = distance;
            result = now;
        }

        for (connection, dist) in connections.get(&now).unwrap() {
            if been.contains(connection) {
                continue;
            }

            been.insert(*connection);
            queue.push_back((*connection, distance + *dist));
        }
    }

    (result, max_distance)
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let node_count = input.trim().parse::<usize>().unwrap();

    let mut connections = HashMap::new();
    // from -> (to, distance)

    for _ in 0..node_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap());

        let node = iter.next().unwrap();
        connections.insert(node, Vec::new());

        loop {
            let other = iter.next().unwrap();
            if other == -1 {
                break;
            }

            let distance = iter.next().unwrap();
            connections.get_mut(&node).unwrap().push((other, distance));
        }
    }

    let (edge, _) = get_farest(1, &connections);
    let (_, result) = get_farest(edge, &connections);

    println!("{}", result);
}
