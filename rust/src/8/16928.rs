use std::io::stdin;
use std::collections::{ HashMap, VecDeque, HashSet };

fn main() {
    let (ladder_count, snake_count) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap());

        (
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    };

    let mut mapping = HashMap::new();
    for _ in 0..(ladder_count + snake_count) {
        let (a, b) = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let mut iter = input
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap());

            (
                iter.next().unwrap(),
                iter.next().unwrap(),
            )
        };

        mapping.insert(a, b);
    }

    let mut queue = VecDeque::new();
    let mut been = HashSet::new();
    queue.push_back((1, 0));
    been.insert(1);
    loop {
        let (now, rolls) = queue.pop_front().unwrap();

        // println!("  {} {} ({} / {})", now, rolls, queue.len(), been.len());

        if now == 100 {
            println!("{}", rolls);
            return;
        }

        for die in 1..=6 {
            let mut will = now+die;

            if been.contains(&will) {
                continue;
            }

            if will > 100 {
                continue;
            }

            if mapping.contains_key(&will) {
                will = *mapping.get(&will).unwrap();
            }

            queue.push_back((will, rolls+1));
            been.insert(will);
        }
    }
}
