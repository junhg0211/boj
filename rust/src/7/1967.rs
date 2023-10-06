use std::io::stdin;
use std::collections::{ HashMap, VecDeque, HashSet };

fn get_far(node: u32, connections: &HashMap::<u32, Vec<(u32, u32)>>) -> (u32, u32) {
    let mut queue = VecDeque::new();
    let mut beens = HashSet::new();
    let mut max_node = 0;
    let mut max_distance = 0;
    queue.push_back((node, 0));
    beens.insert(node);

    while !queue.is_empty() {
        let (node, distance) = queue.pop_front().unwrap();

        if distance > max_distance {
            max_distance = distance;
            max_node = node;
        }

        if !connections.contains_key(&node) {
            continue;
        }

        for (cnode, cdistance) in connections.get(&node).unwrap() {
            if beens.contains(cnode) {
                continue;
            }

            queue.push_back((*cnode, distance + *cdistance));
            beens.insert(*cnode);
        }
    }

    (max_node, max_distance)
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let node_count = input.trim().parse::<usize>().unwrap();

    let mut connections = HashMap::new();
    for _ in 0..node_count-1 {
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

        if !connections.contains_key(&end) {
            connections.insert(end, Vec::new());
        }
        connections.get_mut(&end).unwrap().push((start, weight));
    }

    let (node, _) = get_far(1, &connections);
    let (_, result) = get_far(node, &connections);

    println!("{}", result);
}
