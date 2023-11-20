use std::io::stdin;
use std::collections::BinaryHeap;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let lecture_count = input.trim().parse::<usize>().unwrap();

    let mut lectures = Vec::new();
    for _ in 0..lecture_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap());
        let _id = iter.next().unwrap();
        let start = iter.next().unwrap();
        let end = iter.next().unwrap();

        lectures.push((start, end));
    }
    lectures.sort();

    let mut rooms = BinaryHeap::new();
    for &(start, end) in lectures.iter() {
        if let Some(&thing) = rooms.peek() {
            if u32::MAX - thing <= start {
                rooms.pop();
            }
        }
        rooms.push(u32::MAX - end);
    }

    println!("{:?}", rooms.len());
}
