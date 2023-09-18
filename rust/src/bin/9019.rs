/*
 * https://www.acmicpc.net/problem/9019
 */

use std::io::{ stdin, stdout, BufWriter, Write };
use std::collections::{ VecDeque, HashSet };

fn apply_d(number: i32) -> i32 {
    (number * 2) % 10000
}

fn apply_s(number: i32) -> i32 {
    (number + 9999) % 10000
}

fn apply_l(number: i32) -> i32 {
    (number * 10 + number / 1000) % 10000
}

fn apply_r(number: i32) -> i32 {
    (number / 10 + number % 10 * 1000) % 10000
}

fn tick() {
    let (from, to) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap());

        (
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    };

    let mut been = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back((from, String::new()));
    been.insert(from);
    let mut writer = BufWriter::new(stdout());
    while !queue.is_empty() {
        let (now, history) = queue.pop_front().unwrap();

        if now == to {
            writeln!(writer, "{}", history).unwrap();
            break;
        }

        // println!("  {:4} {} ({} / {})", now, history, queue.len(), been.len());

        let d_result = apply_d(now);
        if !been.contains(&d_result) {
            let mut d_history = history.clone();
            d_history.push('D');
            queue.push_back((d_result, d_history));
            been.insert(d_result);
        }

        let s_result = apply_s(now);
        if !been.contains(&s_result) {
            let mut s_history = history.clone();
            s_history.push('S');
            queue.push_back((s_result, s_history));
            been.insert(s_result);
        }

        let l_result = apply_l(now);
        if !been.contains(&l_result) {
            let mut l_history = history.clone();
            l_history.push('L');
            queue.push_back((l_result, l_history));
            been.insert(l_result);
        }

        let r_result = apply_r(now);
        if !been.contains(&r_result) {
            let mut r_history = history.clone();
            r_history.push('R');
            queue.push_back((r_result, r_history));
            been.insert(r_result);
        }
    }
}

fn main() {
    let count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    for _ in 0..count {
        tick();
    }
}
