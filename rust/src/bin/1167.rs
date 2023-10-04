use std::io::stdin;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let node_count = input.trim().parse::<usize>().unwrap();

    let mut connections: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

    for _ in 0..node_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap());

        let node = iter.next().unwrap();
        connections.insert(node, Vec::new());

        loop {
            let other = iter.next().unwrap();
            if other == -1 {
                break;
            }

            let distance = iter.next().unwrap();
            connections.get_mut(&node).unwrap().push((other, distance));
        }
    }

    println!("{:?}", connections);
}
