use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let string = input.trim().chars().collect::<Vec<_>>();

    let mut counts = vec![vec![0; string.len() + 1]; 26];
    for i in 1..=string.len() {
        let c = string[i - 1];
        for (j, alphabet) in ('a'..='z').enumerate() {
            let add = if c == alphabet { 1 } else { 0 };
            let value = counts[j][i - 1] + add;
            counts[j][i] = value;
        }
    }

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let question_count = input.trim().parse::<usize>().unwrap();

    let mut writer = BufWriter::new(stdout());
    for _ in 0..question_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        let c = iter.next().unwrap().chars().next().unwrap();
        let start = iter.next().unwrap().parse::<usize>().unwrap();
        let end = iter.next().unwrap().parse::<usize>().unwrap();

        let index = (c as u8 - 'a' as u8) as usize;
        let count = counts[index][end + 1] - counts[index][start];
        writeln!(writer, "{}", count).unwrap();
    }
}
