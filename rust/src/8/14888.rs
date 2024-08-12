use std::io::stdin;

fn get_operation(index: usize) -> char {
    ['+', '-', '*', '/'][index]
}

fn backtrack(
    numbers: &Vec<i32>,
    operator_count: &mut Vec<usize>,
    operations: &mut Vec<char>,
) -> (i32, i32) {
    let (mut max, mut min) = (i32::MIN, i32::MAX);

    if numbers.len() == operations.len() + 1 {
        let mut number = numbers[0];
        for (i, &operation) in operations.iter().enumerate() {
            if operation == '+' {
                number += numbers[i + 1];
            } else if operation == '-' {
                number -= numbers[i + 1];
            } else if operation == '*' {
                number *= numbers[i + 1];
            } else if operation == '/' {
                number /= numbers[i + 1];
            }
        }

        max = i32::max(max, number);
        min = i32::min(min, number);
        return (max, min);
    }

    for i in 0..4 {
        if operator_count[i] == 0 {
            continue;
        }

        let operation = get_operation(i);
        operations.push(operation);
        operator_count[i] -= 1;

        let (x, n) = backtrack(numbers, operator_count, operations);
        max = i32::max(max, x);
        min = i32::min(min, n);

        operations.pop().unwrap();
        operator_count[i] += 1;
    }

    (max, min)
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let _ = input.trim().parse::<usize>().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let mut operator_count = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let (max, min) = backtrack(&numbers, &mut operator_count, &mut Vec::new());
    println!("{}\n{}", max, min);
}
