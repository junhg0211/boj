use std::io::stdin;
use std::collections::{ BinaryHeap, HashMap };

fn get_ancient(of: i32, map: &mut HashMap<i32, i32>) -> i32 {
    let mut this = of;
    loop {
        match map.get(&this) {
            Some(thing) => { this = *thing; },
            None => { break; }
        }
    }

    if of != this {
        map.insert(of, this);
    }
    return this;
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

    let mut heap = BinaryHeap::new();
    for _ in 0..edge_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap());
        let start = iter.next().unwrap();
        let end = iter.next().unwrap();
        let weight = iter.next().unwrap();

        heap.push((-weight, start, end));
    }

    let mut parents = HashMap::new();
    let mut result = 0;
    let mut count = 0;
    while count < node_count-2 {
        let (weight, start, end) = match heap.pop() {
            Some(thing) => thing,
            None => break,
        };
        let weight = -weight;

        let start_ancient = get_ancient(start, &mut parents);
        let end_ancient = get_ancient(end, &mut parents);
        if start_ancient == end_ancient {
            continue;
        }

        parents.insert(start_ancient, end_ancient);
        count += 1;
        result += weight;
    }

    println!("{}", result);
}
