use std::io::{ stdin, stdout, BufWriter, Write };

fn main() {
    let message = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().to_owned()
    };

    let mut writer = BufWriter::new(stdout());

    // first line
    for i in 0..message.len() {
        match i % 3 {
            2 => write!(writer, "..*.").unwrap(),
            _ => write!(writer, "..#.").unwrap(),
        }
    }
    writeln!(writer, ".").unwrap();

    // second line
    for i in 0..message.len() {
        match i % 3 {
            2 => write!(writer, ".*.*").unwrap(),
            _ => write!(writer, ".#.#").unwrap(),
        }
    }
    writeln!(writer, ".").unwrap();

    // middle line
    for i in 0..message.len() {
        let c = message.chars().nth(i).unwrap();

        if i == 0 {
            write!(writer, "#.{}.", c).unwrap();
            continue;
        }

        match i % 3 {
            0 => write!(writer, ".{}.", c).unwrap(),
            1 => write!(writer, "#.{}.", c).unwrap(),
            _ => write!(writer, "*.{}.*", c).unwrap(),
        };
    }
    if message.len() % 3 != 0 {
        writeln!(writer, "#").unwrap();
    } else {
        writeln!(writer).unwrap();
    }

    // second line
    for i in 0..message.len() {
        match i % 3 {
            2 => write!(writer, ".*.*").unwrap(),
            _ => write!(writer, ".#.#").unwrap(),
        }
    }
    writeln!(writer, ".").unwrap();

    // first line
    for i in 0..message.len() {
        match i % 3 {
            2 => write!(writer, "..*.").unwrap(),
            _ => write!(writer, "..#.").unwrap(),
        }
    }
    writeln!(writer, ".").unwrap();
}
