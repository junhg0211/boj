use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut writer = BufWriter::new(stdout());

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let command_count = input.trim().parse::<usize>().unwrap();

    let mut stack = Vec::new();
    for _ in 0..command_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut tokens = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap());

        let operation = tokens.next().unwrap();

        if operation == 1 {
            let arg = tokens.next().unwrap();

            stack.push(arg);
            continue;
        }

        if operation == 2 {
            match stack.pop() {
                Some(number) => writeln!(writer, "{}", number).unwrap(),
                None => writeln!(writer, "-1").unwrap(),
            }
            continue;
        }

        if operation == 3 {
            writeln!(writer, "{}", stack.len()).unwrap();
            continue;
        }

        if operation == 4 {
            writeln!(writer, "{}", if stack.is_empty() { 1 } else { 0 }).unwrap();
            continue;
        }

        match stack.last() {
            Some(number) => writeln!(writer, "{}", number).unwrap(),
            None => writeln!(writer, "-1").unwrap(),
        }
    }
}
