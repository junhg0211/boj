use std::io::stdin;

fn factorial(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }

    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }

    return result;
}

fn permutation(index: u64, size: u64) {
    let mut numbers = (1..=size).collect::<Vec<_>>();
    let index = index - 1;

    for i in 0..size {
        let j = size - i - 1;
        let this_index = (index / factorial(j)) as usize % numbers.len();
        let this = numbers.remove(this_index);

        print!("{} ", this);
    }
}

fn index(numbers: &Vec<u64>) {
    let size = numbers.len();
    let mut remainings = (1..=size as u64).collect::<Vec<_>>();

    let mut result = 1;
    for i in 0..size {
        let j = (size - i - 1) as u64;
        let position = remainings
            .iter()
            .position(|x| *x == numbers[i])
            .unwrap();
        result += position as u64 * factorial(j);
        remainings.remove(position);
    }

    println!("{}", result);
}

fn main() {
    // get number count
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<u64>().unwrap();

    // get numbers
    input.clear();
    stdin().read_line(&mut input).unwrap();
    let mut numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let option = numbers.remove(0);

    match option {
        1 => permutation(numbers[0], count),
        _ => index(&mut numbers)
    }
}
