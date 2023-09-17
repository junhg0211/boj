use std::io::stdin;
use std::collections::BinaryHeap;

fn main() {
    let count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    let mut heap = BinaryHeap::new();
    for _ in 0..count {
        let difficulty = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            input.trim().parse::<u32>().unwrap()
        };

        heap.push(difficulty);
    }

    let cuts = (count as f64 * 0.15).round() as u32;
    for _ in 0..cuts {
        heap.pop().unwrap();
    }

    let apply_count = count - cuts * 2;
    let mut sum = 0;
    for _ in 0..apply_count {
        sum += heap.pop().unwrap();
    }
    println!("{}", (sum as f64 / apply_count as f64).round() as u32);
}
