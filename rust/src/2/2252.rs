use std::collections::{HashMap, HashSet, VecDeque};
use std::io::stdin;

fn main() {
    let (student_count, comparison_count) = {
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

    let mut printeds: HashSet<usize> = HashSet::new();
    for i in 1..=student_count {
        let mut deque = VecDeque::new();
        deque.push_back(i);

        while !deque.is_empty() {
            // println!("{} {:?} {:?}", i, printeds, deque);
            let peek = deque[0];

            if printeds.contains(&peek) {
                deque.pop_front().unwrap();
                continue;
            }

            let mut there_were_pointeds = false;
            if let Some(pointeds) = pointedses.get(&peek) {
                for &pointed in pointeds.iter() {
                    if printeds.contains(&pointed) {
                        continue;
                    }

                    there_were_pointeds = true;
                    deque.push_front(pointed);
                }
                pointedses.remove(&peek);
            }

            if there_were_pointeds {
                continue;
            }
            printeds.insert(peek);
            print!("{} ", deque.pop_front().unwrap());

            if let Some(pointings) = pointingses.get(&peek) {
                for &pointing in pointings.iter() {
                    if printeds.contains(&pointing) {
                        continue;
                    }

                    deque.push_back(pointing);
                }
                pointingses.remove(&peek);
            }
        }
    }
    println!();
}
