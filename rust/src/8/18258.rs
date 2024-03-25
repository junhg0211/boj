use std::collections::VecDeque;
use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let testcase_count = input.trim().parse::<usize>().unwrap();

    let mut queue = VecDeque::new();

    let mut writer = BufWriter::new(stdout());
    for _ in 0..testcase_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut tokens = input.trim().split_whitespace();

        let operation = tokens.next().unwrap();

        if operation == "push" {
            let number = tokens.next().unwrap().parse::<u32>().unwrap();
            queue.push_back(number);
            continue;
        }
        if operation == "pop" {
            if let Some(thing) = queue.pop_front() {
                writeln!(writer, "{}", thing).unwrap();
            } else {
                writeln!(writer, "-1").unwrap();
            }
            continue;
        }
        if operation == "size" {
            writeln!(writer, "{}", queue.len()).unwrap();
            continue;
        }
        if operation == "empty" {
            writeln!(writer, "{}", if queue.is_empty() { 1 } else { 0 }).unwrap();
            continue;
        }
        if operation == "front" {
            if let Some(thing) = queue.front() {
                writeln!(writer, "{}", thing).unwrap();
            } else {
                writeln!(writer, "-1").unwrap();
            }
            continue;
        }
        if operation == "back" {
            if let Some(thing) = queue.back() {
                writeln!(writer, "{}", thing).unwrap();
            } else {
                writeln!(writer, "-1").unwrap();
            }
            continue;
        }
    }
}
