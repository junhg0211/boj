use std::io::stdin;
use std::collections::{ HashMap, HashSet, BinaryHeap };

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let _city_count = input.trim().parse::<usize>().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let bus_count = input.trim().parse::<usize>().unwrap();

    let mut buses = HashMap::new();
    for _ in 0..bus_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap());

        let from = iter.next().unwrap();
        let to = iter.next().unwrap();
        let fee = iter.next().unwrap();

        if !buses.contains_key(&from) {
            buses.insert(from, Vec::new());
        }

        buses.get_mut(&from).unwrap().push((to, fee));
    }

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());

    let start = iter.next().unwrap();
    let end = iter.next().unwrap();

    // --- dijkstra

    let mut distances = HashMap::new();
    let mut beens = HashSet::new();
    let mut heap = BinaryHeap::new();
    distances.insert(start, 0);
    heap.push((u32::MAX, start));

    while !heap.is_empty() {
        // get minimal index
        let mut min_distance = u32::MAX;
        let mut min_position = 0;
        let mut stop = false;
        loop {
            if heap.is_empty() {
                stop = true;
                break;
            }

            let (mut distance, position) = heap.pop().unwrap();
            distance = u32::MAX - distance;

            if beens.contains(&position) {
                continue;
            }

            min_distance = distance;
            min_position = position;
            break;
        }

        if stop {
            break;
        }

        // update distances
        beens.insert(min_position);

        if !buses.contains_key(&min_position) {
            continue;
        }

        for (destination, fee) in buses.get(&min_position).unwrap() {
            let this_cost = min_distance + fee;

            if !distances.contains_key(destination) {
                distances.insert(*destination, this_cost);
                heap.push((u32::MAX - this_cost, *destination));
                continue;
            }

            let now_cost = distances.get(destination).unwrap();
            if this_cost < *now_cost {
                distances.insert(*destination, this_cost);
                heap.push((u32::MAX - this_cost, *destination));
            }
        }
    }

    println!("{}", distances.get(&end).unwrap());
}
