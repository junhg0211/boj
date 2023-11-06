use std::{
    io::stdin,
    collections::{ HashMap, BinaryHeap },
};

fn get_ancient(this: i32, map: &mut HashMap<i32, i32>) -> i32 {
    let mut of = this;
    while let Some(&thing) = map.get(&of) {
        of = thing;
    }
    if this != of {
        map.insert(this, of);
    }

    return of;
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

    // -- kruskal
    let mut parent = HashMap::new();
    let mut result = 0;
    while parent.len() < node_count-1 {
        let (weight, start, end) = heap.pop().unwrap();
        let weight = -weight;

        let start_ancient = get_ancient(start, &mut parent);
        let end_ancient = get_ancient(end, &mut parent);

        if start_ancient == end_ancient {
            continue;
        }

        parent.insert(start_ancient, end_ancient);
        result += weight;
    }

    println!("{}", result);
}
