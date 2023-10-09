use std::io::stdin;
use std::collections::{ HashMap, HashSet, BinaryHeap };

fn dijkstra(start: usize, paths: &HashMap<usize, Vec<(usize, usize)>>)
        -> HashMap::<usize, usize> {
    let mut distances = HashMap::new();
    let mut beens = HashSet::new();
    let mut stack = BinaryHeap::new();

    stack.push((usize::MAX, start));
    while !stack.is_empty() {
        // -- get least distance village
        let mut stop = false;
        let (min_village, min_distance) = loop {
            if stack.is_empty() {
                stop = true;
                break (0, 0);
            }

            let (fake_distance, village) = stack.pop().unwrap();
            let distance = usize::MAX - fake_distance;

            if beens.contains(&village) {
                continue;
            }

            break (village, distance);
        };

        if stop {
            break;
        }

        beens.insert(min_village);

        if !paths.contains_key(&min_village) {
            continue;
        }

        for (to_village, to_distance) in paths.get(&min_village).unwrap() {
            let this_distance = min_distance + *to_distance;

            if !distances.contains_key(to_village) {
                distances.insert(*to_village, this_distance);
                stack.push((usize::MAX - this_distance, *to_village));
                continue;
            }

            let original_distance = distances.get(to_village).unwrap();
            if &this_distance < original_distance {
                distances.insert(*to_village, this_distance);
                stack.push((usize::MAX - this_distance, *to_village));
                continue;
            }
        }
    }

    distances
}

fn main() {
    // -- get meta numbers
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let student_count = iter.next().unwrap();
    let path_count = iter.next().unwrap();
    let party_village = iter.next().unwrap();

    // -- get paths
    let mut paths = HashMap::new();
    for _ in 0..path_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());
        let start = iter.next().unwrap();
        let end = iter.next().unwrap();
        let duration = iter.next().unwrap();

        if !paths.contains_key(&start) {
            paths.insert(start, Vec::new());
        }

        paths
            .get_mut(&start)
            .unwrap()
            .push((end, duration));
    }

    // -- dijkstra
    let distances = dijkstra(party_village, &paths);
    let mut max_distance = 0;

    for village in 1..=student_count {
        if village == party_village {
            continue;
        }

        let result = dijkstra(village, &paths);

        let return_distance = result.get(&party_village).unwrap();
        let go_distance = distances.get(&village).unwrap();
        let distance = return_distance + go_distance;

        if distance > max_distance {
            max_distance = distance;
        }
    }

    // -- print result
    println!("{}", max_distance);
}
