use std::io::{stdin, stdout, BufWriter, Write};

fn not(a: u32, b: u32) -> u32 {
    if a == 1 && b == 2 || a == 2 && b == 1 {
        return 3;
    }

    if a == 1 && b == 3 || a == 3 && b == 1 {
        return 2;
    }

    return 1;
}

fn hanoi(number: u32, from: u32, to: u32, writer: &mut BufWriter<std::io::Stdout>) {
    if number == 0 {
        return;
    }

    let middle = not(from, to);
    hanoi(number - 1, from, middle, writer);
    writeln!(writer, "{} {}", from, to).unwrap();
    hanoi(number - 1, middle, to, writer);
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let number = input.trim().parse::<u32>().unwrap();

    let mut writer: BufWriter<std::io::Stdout> = BufWriter::new(stdout());
    writeln!(writer, "{}", u32::pow(2, number) - 1).unwrap();
    hanoi(number, 1, 3, &mut writer);
}
