use std::io::{ stdin, stdout, Write, BufWriter };

fn tick() {
    let message = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().to_owned()
    };

    let side = (message.len() as f64).sqrt() as usize;

    let mut writer = BufWriter::new(stdout());
    for i in 0..message.len() {
        let x = i % side;
        let y = i / side;

        let index = x * side + (side-y) - 1;
        let c = message.chars().nth(index).unwrap();
        write!(writer, "{c}").unwrap();
    }
    writeln!(writer).unwrap();
}

fn main() {
    let count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    for _ in 0..count {
        tick();
    }
}
