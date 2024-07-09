use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let sentence = input.chars().collect::<Vec<_>>();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let key = input.trim().chars().collect::<Vec<_>>();

    // println!("{:?}", sentence);

    let mut writer = BufWriter::new(stdout());
    for i in 0..sentence.len() {
        let c = sentence[i] as u8;

        if c == ' ' as u8 {
            write!(writer, " ").unwrap();
            continue;
        }
        if c == '\n' as u8 {
            break;
        }

        let k = key[i % key.len()] as u8;

        let mut result = 'a' as u8 + 25 + c - k;
        while result > 'z' as u8 {
            result -= 26;
        }

        write!(writer, "{}", result as char).unwrap();
    }

    writeln!(writer).unwrap();
}
