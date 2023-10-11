use std::io::{ stdin, stdout, Write, BufWriter };
use std::collections::{ HashMap, HashSet, VecDeque };

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());

    let node_count = iter.next().unwrap();
    let edge_count = iter.next().unwrap();
    let start_node = iter.next().unwrap();

    let mut connections = HashMap::<u32, Vec<u32>>::new();
    for _ in 0..edge_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap());

        let a = iter.next().unwrap();
        let b = iter.next().unwrap();

        match connections.get_mut(&b) {
            Some(thing) => match thing.binary_search(&a) {
                Ok(pos) => thing.insert(pos, a),
                Err(pos) => thing.insert(pos, a),
            },
            None => { connections.insert(b, vec![a]); },
        }

        match connections.get_mut(&a) {
            Some(thing) => match thing.binary_search(&b) {
                Ok(pos) => thing.insert(pos, b),
                Err(pos) => thing.insert(pos, b),
            },
            None => { connections.insert(a, vec![b]); },
        }
    }

    let mut order = HashMap::<u32, u32>::new();
    let mut queue = VecDeque::new();
    let mut beens = HashSet::new();
    queue.push_back(start_node);
    beens.insert(start_node);
    let mut index = 1;

    loop {
        let now = match queue.pop_front() {
            Some(thing) => thing,
            None => break,
        };

        order.insert(now, index);
        index += 1;

        let connection = match connections.get(&now) {
            Some(thing) => thing,
            None => continue,
        };

        for node in connection {
            if beens.contains(node) {
                continue;
            }

            queue.push_back(*node);
            beens.insert(*node);
        }
    }

    let mut writer = BufWriter::new(stdout());
    for i in 1..=node_count {
        writeln!(writer, "{}", match order.get(&i) {
            Some(thing) => *thing,
            None => 0,
        }).unwrap();
    }
}
