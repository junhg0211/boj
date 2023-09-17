use std::io::{ stdin, stdout, BufWriter, Write };
use std::collections::HashSet;

fn main() {
    let _count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<i32>().unwrap()
    };

    let numbers = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<HashSet<_>>()
    };

    let _question_count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<i32>().unwrap()
    };

    let questions = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    };

    let mut writer = BufWriter::new(stdout());
    for question in questions {
        match numbers.contains(&question) {
            true => writeln!(writer, "1").unwrap(),
            false => writeln!(writer, "0").unwrap(),
        }
    }
}
