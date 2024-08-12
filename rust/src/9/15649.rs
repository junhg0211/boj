use std::io::{stdin, stdout, BufWriter, Stdout, Write};

fn backtrack(number: u32, count: u32, numbers: &mut Vec<u32>, writer: &mut BufWriter<Stdout>) {
    for i in 1..=number {
        if numbers.contains(&i) {
            continue;
        }

        numbers.push(i);

        if count == 1 {
            for &n in numbers.iter() {
                write!(writer, "{} ", n).unwrap();
            }
            writeln!(writer).unwrap();
        } else {
            backtrack(number, count - 1, numbers, writer);
        }

        numbers.pop().unwrap();
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    let number = iter.next().unwrap();
    let count = iter.next().unwrap();

    let mut writer = BufWriter::new(stdout());
    backtrack(number, count, &mut Vec::new(), &mut writer);
}
