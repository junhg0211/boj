use std::io::stdin;
use std::collections::{ HashMap, HashSet };

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let node_count = input.trim().parse::<i32>().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap());
    let mut connections = HashMap::new();
    let mut root = 0;
    for i in 0..node_count {
        let destination = iter.next().unwrap();

        // destination -> i
        if !connections.contains_key(&destination) {
            connections.insert(destination, Vec::new());
        }
        connections.get_mut(&destination).unwrap().push(i);

        if destination == -1 {
            root = i as i32;
        }
    }

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let deletion = input.trim().parse::<i32>().unwrap();

    let mut been = HashSet::new();
    let mut stack = Vec::new();
    been.insert(deletion);

    if !been.contains(&root) {
        stack.push(root);
        been.insert(root);
    }

    let mut leap_count = 0;
    while !stack.is_empty() {
        let now = stack.pop().unwrap();

        if !connections.contains_key(&now) {
            leap_count += 1;
            continue;
        }

        for connection in connections.get(&now).unwrap() {
            if connection == &deletion && connections.get(&now).unwrap().len() == 1 {
                leap_count += 1;
                continue;
            }

            if been.contains(&connection) {
                continue;
            }

            stack.push(*connection);
            been.insert(*connection);
        }
    }

    println!("{}", leap_count);
}
