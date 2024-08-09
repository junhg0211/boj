use std::{
    collections::HashMap,
    io::{stdin, stdout, BufWriter, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let word_count = iter.next().unwrap();
    let length_min = iter.next().unwrap();

    let mut words = HashMap::new();
    for _ in 0..word_count {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let word = input.trim().to_string();

        if word.len() < length_min {
            continue;
        }

        if words.contains_key(&word) {
            let data: &mut (usize, usize, String) = words.get_mut(&word).unwrap();
            data.0 -= 1;
        } else {
            words.insert(
                word.to_string(),
                (usize::MAX - 1, usize::MAX - word.len(), word.to_string()),
            );
        }
    }

    let mut words = words.values().collect::<Vec<_>>();
    words.sort();

    let mut writer = BufWriter::new(stdout());
    for i in 0..words.len() {
        // println!("{}", words[i].2);
        writeln!(writer, "{}", words[i].2).unwrap();
    }
}
