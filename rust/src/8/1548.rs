use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let number_count = input.trim().parse::<usize>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    numbers.sort();

    if number_count <= 1 {
        println!("{}", number_count);
        return;
    }

    let mut result = 0;
    for i in 0..=number_count-2 {
        let small2 = numbers[i] + numbers[i+1];

        for j in i..number_count {
            let length = j-i+1;
            if small2 > numbers[j] && length > result {
                result = length;
            }
        }
    }

    println!("{}", result);
}
