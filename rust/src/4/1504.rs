use std::io::stdin;
use std::collections::{ HashMap, HashSet, BinaryHeap };
use std::cmp::min;

fn dijkstra(from: u32, connections: &HashMap<u32, Vec<(u32, u32)>>) -> HashMap<u32, u32> {
    let mut distances = HashMap::new();
    let mut beens = HashSet::new();
    let mut stack = BinaryHeap::new();

    distances.insert(from, 0);
    stack.push((u32::MAX, from));
    loop {
        let mut min_distance = 0;
        let mut min_node = 0;
        let mut stop = false;
        loop {
            let (reverse_distance, node) = match stack.pop() {
                Some(thing) => thing,
                None => {
                    stop = true;
                    break;
                },
            };

            if beens.contains(&node) {
                continue;
            }

            let distance = u32::MAX - reverse_distance;

            min_distance = distance;
            min_node = node;

            break;
        }

        if stop {
            break;
        }

        beens.insert(min_node);

        if !connections.contains_key(&min_node) {
            continue;
        }

        for (to_node, to_distance) in connections.get(&min_node).unwrap() {
            let total_distance = *to_distance + min_distance;
            match distances.get(to_node) {
                None => {
                    distances.insert(*to_node, total_distance);
                },
                Some(thing) => {
                    if &total_distance < thing {
                        distances.insert(*to_node, total_distance);
                    }
                },
            }

            if beens.contains(to_node) {
                continue;
            }

            stack.push((u32::MAX - total_distance, *to_node));
        }
    }

    distances
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let node_count = iter.next().unwrap();
    let edge_count = iter.next().unwrap();

    let mut connections = HashMap::<u32, Vec<(u32, u32)>>::new();
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

        match connections.get_mut(&start) {
            Some(thing) => thing.push((end, weight)),
            None => {
                connections.insert(start, vec![(end, weight)]);
            },
        };

        match connections.get_mut(&end) {
            Some(thing) => thing.push((start, weight)),
            None => {
                connections.insert(end, vec![(start, weight)]);
            },
        };
    }

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let layovers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    // dijkstra
    let start_distances = dijkstra(1, &connections);
    let layover1_distances = dijkstra(layovers[0], &connections);
    let layover2_distances = dijkstra(layovers[1], &connections);

    let one_no = !start_distances.contains_key(&layovers[0])
        || !layover1_distances.contains_key(&layovers[1])
        || !layover2_distances.contains_key(&(node_count as u32));
    let two_no = !start_distances.contains_key(&layovers[1])
        || !layover2_distances.contains_key(&layovers[0])
        || !layover1_distances.contains_key(&(node_count as u32));

    let mut distance1 = u32::MAX;
    let mut distance2 = u32::MAX;
    if one_no && two_no {
        println!("-1");
        return;
    } else if one_no {
        distance2 = start_distances[&layovers[1]] + layover2_distances[&layovers[0]] + layover1_distances[&(node_count as u32)];
    } else if two_no {
        distance1 = start_distances[&layovers[0]] + layover1_distances[&layovers[1]] + layover2_distances[&(node_count as u32)];
    } else {
        distance1 = start_distances[&layovers[0]] + layover1_distances[&layovers[1]] + layover2_distances[&(node_count as u32)];
        distance2 = start_distances[&layovers[1]] + layover2_distances[&layovers[0]] + layover1_distances[&(node_count as u32)];
    }

    println!("{}", min(distance1, distance2));
}
