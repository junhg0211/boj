use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::{stdin, stdout, BufWriter, Write};

fn reverse(number: usize) -> usize {
    usize::MAX - number
}

fn main() {
    let (node_count, comparison_count) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };

    // let mut ordering = HashMap::<usize, (HashSet<usize>, HashSet<usize>)>::new();
    let mut pointingses = HashMap::<usize, HashSet<usize>>::new();
    let mut pointedses = HashMap::<usize, HashSet<usize>>::new();

    for _ in 0..comparison_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());

        let a = iter.next().unwrap();
        let b = iter.next().unwrap();

        if !pointingses.contains_key(&a) {
            pointingses.insert(a, HashSet::new());
        }
        pointingses.get_mut(&a).unwrap().insert(b);

        if !pointedses.contains_key(&b) {
            pointedses.insert(b, HashSet::new());
        }
        pointedses.get_mut(&b).unwrap().insert(a);
    }

    let mut heap = BinaryHeap::new();
    for node in 1..=node_count {
        if !pointedses.contains_key(&node) {
            heap.push(reverse(node));
        }
    }

    let mut writer = BufWriter::new(stdout());
    let mut printeds: HashSet<usize> = HashSet::new();
    while !heap.is_empty() {
        // debug: print stack
        /* {
            print!("[");
            for &i in heap.iter() {
                print!("{} ", reverse(i));
            }
            println!("]");
        } */

        let now = reverse(heap.pop().unwrap());

        if printeds.contains(&now) {
            continue;
        }

        let real_pointeds = match pointedses.get(&now) {
            Some(pointeds) => {
                let mut real_pointeds = Vec::new();
                for &pointed in pointeds.iter() {
                    if printeds.contains(&pointed) {
                        continue;
                    }

                    real_pointeds.push(pointed);
                }
                real_pointeds
            }
            _ => Vec::new(),
        };

        if !real_pointeds.is_empty() {
            for real_pointed in real_pointeds {
                heap.push(reverse(real_pointed));
            }
            continue;
        }

        pointedses.remove(&now);
        write!(writer, "{} ", now).unwrap();
        printeds.insert(now);

        let real_pointings = match pointingses.get(&now) {
            Some(pointings) => {
                let mut real_pointings = Vec::new();
                for &pointing in pointings.iter() {
                    if printeds.contains(&pointing) {
                        continue;
                    }

                    real_pointings.push(pointing);
                }
                real_pointings
            }
            _ => Vec::new(),
        };

        for real_pointing in real_pointings {
            if let Some(pointeds) = pointedses.get_mut(&real_pointing) {
                pointeds.remove(&now);
                if pointeds.is_empty() {
                    pointedses.remove(&real_pointing);
                }
            }

            if let Some(pointeds) = pointedses.get(&real_pointing) {
                if pointeds.len() > 0 {
                    continue;
                }
            }

            heap.push(reverse(real_pointing));
        }
    }
    writeln!(writer).unwrap();
}
