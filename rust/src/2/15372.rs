use std::io::{ stdin, stdout, Write, BufWriter };

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<usize>().unwrap();

    let mut writer = BufWriter::new(stdout());
    for _ in 0..count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let n = input.trim().parse::<u64>().unwrap();

        writeln!(writer, "{}", n * n).unwrap();
    }
}
