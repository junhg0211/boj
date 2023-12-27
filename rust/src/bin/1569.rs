use std::io::stdin;
use std::cmp::{ min, max };

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<usize>().unwrap();

    let mut points = Vec::new();
    for _ in 0..count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap());

        let x = iter.next().unwrap();
        let y = iter.next().unwrap();

        points.push((x, y));
    }

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for &(x, y) in points.iter() {
        if (x != min_x && x != max_x)
        && (y != min_y && y != max_y) {
            // println!("-1");
            println!("-1 {} {}", x, y);
            return;
        }

        min_x = min(x, min_x);
        min_y = min(y, min_y);
        max_x = max(x, max_x);
        max_y = max(y, max_y);
    }

    if max_x - min_x != max_y - min_y {
        println!("-1");
        return;
    }

    println!("{}", max_x - min_x);
}
