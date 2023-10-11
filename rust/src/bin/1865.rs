use std::io::stdin;
use std::collections::HashMap;

fn bf(start: i32, node_count: usize, connections: &HashMap<i32, Vec<(i32, i32)>>) -> bool {
    let mut distances = HashMap::new();
    distances.insert(start, 0);

    for i in 1..=node_count {
        for node in 1..=node_count as i32 {
            let until_this_distance = match distances.get(&node) {
                Some(thing) => *thing,
                None => continue,
            };

            let connection = match connections.get(&node) {
                Some(thing) => thing,
                None => continue,
            };

            for (connected_node, distance) in connection {
                let this_distance = until_this_distance + *distance;

                let original_distance = match distances.get(connected_node) {
                    Some(thing) => *thing,
                    None => {
                        distances.insert(*connected_node, this_distance);
                        continue;
                    }
                };

                if this_distance < original_distance {
                    distances.insert(*connected_node, this_distance);

                    if i == node_count {
                        // println!("{} {} {} {}", node, connected_node, original_distance, this_distance);
                        return true;
                    }
                }
            }
        }
    }

    return false;
}

fn tick() {
    // -- get data
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let node_count = iter.next().unwrap();
    let path_count = iter.next().unwrap();
    let wormhole_count = iter.next().unwrap();

    let mut connections: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    for _ in 0..path_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap());

        let start = iter.next().unwrap();
        let end = iter.next().unwrap();
        let weight = iter.next().unwrap();

        match connections.get_mut(&start) {
            Some(thing) => thing.push((end, weight)),
            None => { connections.insert(start, vec![(end, weight)]); },
        }

        match connections.get_mut(&end) {
            Some(thing) => thing.push((start, weight)),
            None => { connections.insert(start, vec![(start, weight)]); },
        }
    }

    for _ in 0..wormhole_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap());

        let start = iter.next().unwrap();
        let end = iter.next().unwrap();
        let weight = -iter.next().unwrap();

        match connections.get_mut(&start) {
            Some(thing) => thing.push((end, weight)),
            None => { connections.insert(start, vec![(end, weight)]); },
        }
    }

    // -- bellman-ford
    let mut negative_infinite = false;
    for i in 1..=node_count as i32 {
        negative_infinite = bf(i, node_count, &connections) || negative_infinite;
        if negative_infinite {
            break;
        }
    }

    println!("{}", if negative_infinite { "YES" } else { "NO" });
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let testcase_count = input.trim().parse::<usize>().unwrap();
    for _ in 0..testcase_count {
        tick();
    }
}
