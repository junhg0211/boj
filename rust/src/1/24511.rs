use std::{
    collections::VecDeque,
    io::{stdin, stdout, BufWriter, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let ds_type = input
        .trim()
        .split_whitespace()
        .map(|x| x == "1")
        .collect::<Vec<_>>();

    let mut queue = VecDeque::new();
    input.clear();
    stdin().read_line(&mut input).unwrap();
    for (i, number) in input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .enumerate()
    {
        if !ds_type[i] {
            queue.push_front(number);
        }
    }

    input.clear();
    stdin().read_line(&mut input).unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let mut writer = BufWriter::new(stdout());
    for number in input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
    {
        queue.push_back(number);
        write!(writer, "{} ", queue.pop_front().unwrap()).unwrap();
    }
    writeln!(writer).unwrap();
}
