use std::io::stdin;
use std::collections::BinaryHeap;

fn main() {
    // get points count
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let point_count = input.trim().parse::<usize>().unwrap();

    // get points
    let mut points = Vec::new();
    for _ in 0..point_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<f64>().unwrap());
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        points.push((x, y));
    }

    // get distances
    let mut distances = BinaryHeap::new();
    for i in 0..point_count {
        for j in i+1..point_count {
            let dx = points[j].0 - points[i].0;
            let dy = points[j].1 - points[i].1;
            let distance = (dx*dx + dy*dy).sqrt();

            distances.push((-distance, i, j));
        }
    }

    println!("{:?}", distances);
}
