use std::io::stdin;
use std::collections::VecDeque;

fn main() {
    // get numbers
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let numbers: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let now = numbers[0];
    let target = numbers[1];

    // bfs
    let mut queue: VecDeque<(u64, u64)> = VecDeque::new();
    queue.push_back((0, now));
    let mut result = i32::max_value();
    loop {
        // check empty
        if queue.is_empty() {
            break;
        }

        // get numbers
        let now = queue.pop_front().unwrap();
        let count = now.0;
        let now = now.1;

        // if invalid
        if now > target {
            continue;
        }

        // if answer
        if now == target {
            let count = count as i32;
            result = if count < result { count } else { result };
        }

        // push next
        if now * 2 <= target {
            queue.push_back((count + 1, now * 2));
        }
        if now * 10 + 1 <= target {
            queue.push_back((count + 1, now * 10 + 1));
        }
    }

    // print result
    if result == i32::max_value() {
        println!("{}", -1);
    } else {
        println!("{}", result + 1);
    }
}
