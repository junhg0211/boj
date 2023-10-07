use std::io::{ stdin, stdout, Write, BufWriter };
use std::collections::{ HashMap, HashSet, BinaryHeap };

fn main() {
    // -- get node and edge count
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let node_count = iter.next().unwrap();
    let edge_count = iter.next().unwrap();

    // -- get start node number
    input.clear();
    stdin().read_line(&mut input).unwrap();
    let start_node = input.trim().parse::<u32>().unwrap();

    // -- get edges
    let mut connections = HashMap::new();
    for _ in 0..edge_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap());

        let start = iter.next().unwrap();
        let end = iter.next().unwrap();
        let weight = iter.next().unwrap();

        if !connections.contains_key(&start) {
            connections.insert(start, Vec::new());
        }
        connections.get_mut(&start).unwrap().push((end, weight));
    }

    // -- dijkstra
    let mut distances = HashMap::new();
    let mut beens = HashSet::new();
    let mut queue = BinaryHeap::new();

    distances.insert(start_node, 0);
    queue.push((u32::MAX, start_node));
    while !queue.is_empty() {
        // -- get min node
        let (mut distance, mut node) = (0, 0);
        let mut stop = false;
        loop {
            if queue.is_empty() {
                stop = true;
                break;
            }

            (distance, node) = queue.pop().unwrap();
            distance = u32::MAX - distance;
            break;
        }

        if stop {
            break;
        }

        if beens.contains(&node) {
            continue;
        }

        beens.insert(node);

        // -- visit
        if !connections.contains_key(&node) {
            continue;
        }

        for (cnode, cdistance) in connections.get(&node).unwrap() {
            let this_distance = distance + *cdistance;

            if !distances.contains_key(cnode) {
                distances.insert(*cnode, this_distance);
                queue.push((u32::MAX - this_distance, *cnode));
                continue;
            }

            let original_distance = distances.get(cnode).unwrap();
            if original_distance > &this_distance {
                distances.insert(*cnode, this_distance);
                queue.push((u32::MAX - this_distance, *cnode));
            }
        }
    }

    // -- print result
    let mut writer = BufWriter::new(stdout());
    for i in 1..=node_count {
        match distances.get(&(i as u32)) {
            Some(distance) => writeln!(writer, "{}", distance).unwrap(),
            None => writeln!(writer, "INF").unwrap()
        }
    }
}
