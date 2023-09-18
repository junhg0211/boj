use std::io::{ stdin, stdout, Write, BufWriter };
use std::collections::BinaryHeap;

fn main() {
    let count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    let mut writer = BufWriter::new(stdout());
    let mut heap: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new();
    for _ in 0..count {
        let number = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            input.trim().parse::<i32>().unwrap()
        };

        if number == 0 {
            match heap.pop() {
                Some(r) => writeln!(writer, "{}", r.2).unwrap(),
                _ => writeln!(writer, "0").unwrap(),
            }
        } else {
            heap.push((-number.abs(), -number, number));
        }

        // println!("{:?}", heap);
    }
}
