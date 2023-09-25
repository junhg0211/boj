use std::io::stdin;
use std::collections::BinaryHeap;

fn main() {
    let (shortcut_count, target) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap());

        (
            iter.next().unwrap(),
            iter.next().unwrap(),
            )
    };

    let mut shortcuts = Vec::new();
    for _ in 0..shortcut_count {
        let shortcut = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let mut iter = input
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap());

            (
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            )
        };

        shortcuts.push(shortcut);
    }

    let mut heap = BinaryHeap::new();
    heap.push((0, 0));

    while !heap.is_empty() {
        let (neg_cost, now) = heap.pop().unwrap();

        // println!("{} {}", -neg_cost, now);

        if now > target {
            continue;
        }

        if now == target {
            println!("{}", -neg_cost);
            return;
        }

        for (start, end, cost) in &shortcuts {
            if *start < now {
                continue;
            }

            let new_cost = cost + (-neg_cost) + start - now;

            heap.push((-new_cost, *end));
        }

        heap.push((neg_cost - (target-now), target));
    }
}
