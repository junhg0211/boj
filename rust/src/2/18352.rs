use std::io::{ stdin, stdout, Write, BufWriter };
use std::collections::{ HashMap, VecDeque, HashSet };

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    let _city_count = iter.next().unwrap();
    let path_count = iter.next().unwrap();
    let target_distance = iter.next().unwrap();
    let start_city = iter.next().unwrap();

    let mut connections = HashMap::new();
    for _ in 0..path_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap());

        let from = iter.next().unwrap();
        let to = iter.next().unwrap();

        if !connections.contains_key(&from) {
            connections.insert(from, Vec::new());
        }

        connections.get_mut(&from).unwrap().push(to);
    }

    let mut result_cities = Vec::new();
    let mut queue = VecDeque::new();
    let mut been = HashSet::new();
    queue.push_back((start_city, 0));
    been.insert(start_city);
    while !queue.is_empty() {
        let (now, until_distance) = queue.pop_front().unwrap();

        if until_distance == target_distance {
            result_cities.push(now);
            continue;
        }

        if !connections.contains_key(&now) {
            continue;
        }

        for connection in connections.get(&now).unwrap() {
            if been.contains(connection) {
                continue;
            }

            queue.push_back((*connection, until_distance + 1));
            been.insert(*connection);
        }
    }

    let mut writer = BufWriter::new(stdout());
    if result_cities.is_empty() {
        writeln!(writer, "-1").unwrap();
        return;
    }

    result_cities.sort();
    for city in result_cities {
        writeln!(writer, "{}", city).unwrap();
    }
}
