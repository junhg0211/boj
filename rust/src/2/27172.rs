use std::io::{ stdin, stdout, Write, BufWriter };
use std::collections::{ HashSet, HashMap };

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let _count = input.trim().parse::<usize>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut number_set = HashSet::new();
    let mut max_number = 0;
    for &number in &numbers {
        number_set.insert(number);

        if number > max_number {
            max_number = number;
        }
    }

    let mut scores = HashMap::<u32, i32>::new();
    for number in &numbers {
        for multiple in (number*2..=max_number).step_by((*number) as usize) {
            if number_set.contains(&multiple) {
                scores.insert(*number, *scores.get(number).unwrap_or(&0) + 1);
                scores.insert(multiple, *scores.get(&multiple).unwrap_or(&0) - 1);
            }
        }
    }

    let mut writer = BufWriter::new(stdout());
    for number in &numbers {
        write!(writer, "{} ", scores.get(number).unwrap_or(&0)).unwrap();
    }
    writeln!(writer).unwrap();
}
