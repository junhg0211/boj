use std::io::{ stdin, stdout, BufWriter, Write };

fn swap(a: usize, b: usize, vector: &mut Vec::<i32>) {
    let tmp = vector[a];
    vector[a] = vector[b];
    vector[b] = tmp;
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let number_count = input.trim().parse::<usize>().unwrap();

    let mut numbers = Vec::new();
    for _ in 0..number_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let number = input.trim().parse::<i32>().unwrap();
        numbers.push(number);
    }

    for i in 0..number_count {
        for j in i+1..number_count {
            if numbers[i] > numbers[j] {
                swap(i, j, &mut numbers);
            }
        }
    }

    let mut writer = BufWriter::new(stdout());
    for number in numbers {
        writeln!(writer, "{}", number).unwrap();
    }
}
