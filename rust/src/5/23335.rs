use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let _ = input.trim().parse::<usize>().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());

    let mut writer = BufWriter::new(stdout());
    for number in numbers {
        let mut sum = 1;
        for i in 2..=(number as f64).sqrt().floor() as u32 {
            if number % i == 0 {
                sum += i;
                sum += number / i;

                if number == i * i {
                    sum -= i;
                }
            }
        }

        if number == 1 {
            sum = 0;
        }

        if sum > number {
            writeln!(writer, "abundant").unwrap();
        } else if sum == number {
            writeln!(writer, "perfect").unwrap();
        } else {
            writeln!(writer, "deficient").unwrap();
        }
    }
}
