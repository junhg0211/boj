use std::io::{ stdin, BufWriter, Write, stdout };

fn main() {
    let count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    let mut writer = BufWriter::new(stdout());
    for _ in 0..count {
        let (a, b) = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let mut iter = input
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap());

            (
                iter.next().unwrap(),
                iter.next().unwrap(),
            )
        };

        writeln!(writer, "{}", a + b).unwrap();
    }
}
