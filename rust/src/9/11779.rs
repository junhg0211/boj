use std::io::{ stdin, stdout, Write, BufWriter };
use std::collections::{ HashMap, HashSet, BinaryHeap };

fn main() {
    let mut input: String = String::new();
    stdin().read_line(&mut input).unwrap();
    let _node_count: usize = input.trim().parse().unwrap();

    let mut input: String = String::new();
    stdin().read_line(&mut input).unwrap();
    let edge_count: usize = input.trim().parse().unwrap();

    let mut connections: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
    for _ in 0..edge_count {
        let mut input: String = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap());

        let from = iter.next().unwrap();
        let to = iter.next().unwrap();
        let weight = iter.next().unwrap();

        match connections.get_mut(&from) {
            Some(thing) => thing.push((to, weight)),
            None => { connections.insert(from, vec![(to, weight)]); },
        };
    }

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());

    let start_node = iter.next().unwrap();
    let end_node = iter.next().unwrap();

    // -- dijkstra
    let mut distances: HashMap<u32, (u32, u32)> = HashMap::new();
    let mut beens: HashSet<u32> = HashSet::new();
    let mut heap: BinaryHeap<(u32, u32)> = BinaryHeap::new();
    heap.push((u32::MAX, start_node));

    loop {
        let mut min_distance = 0;
        let mut min_node = 0;
        let mut stop = false;
        loop {
            let (reverse_distance, node) = match heap.pop() {
                Some(thing) => thing,
                None => {
                    stop = true;
                    break;
                },
            };

            let distance = u32::MAX - reverse_distance;
            min_distance = distance;
            min_node = node;

            break;
        }

        if stop {
            break;
        }

        if beens.contains(&min_node) {
            continue;
        }
        beens.insert(min_node);

        if !connections.contains_key(&min_node) {
            continue;
        }

        for (to_node, to_distance) in connections.get(&min_node).unwrap() {
            let total_distance = min_distance + *to_distance;

            match distances.get_mut(to_node) {
                Some((thing, previous)) => {
                    if total_distance < *thing {
                        *thing = total_distance;
                        *previous = min_node;
                    }
                },
                None => {
                    distances.insert(*to_node, (total_distance, min_node));
                },
            };

            heap.push((u32::MAX - total_distance, *to_node));
        }
    }

    // -- get path
    let mut path = vec![end_node];
    loop {
        let last = path.last().unwrap();
        let previous = distances.get(&last).unwrap().1;
        path.push(previous);

        if previous == start_node {
            break;
        }
    }

    // -- print result
    let mut writer = BufWriter::new(stdout());
    writeln!(writer, "{}", distances.get(&end_node).unwrap().0).unwrap();
    writeln!(writer, "{}", path.len()).unwrap();
    loop {
        match path.pop() {
            Some(thing) => write!(writer, "{} ", thing).unwrap(),
            None => break,
        };
    }
    writeln!(writer).unwrap();
}
