use std::io::stdin;
use std::cmp::max;
use std::collections::HashSet;

type Decimal = Vec::<i64>;

fn decimalify(mut number: i64) -> Decimal {
    let mut result = Vec::new();
    while number > 0 {
        result.push(number % 10);
        number /= 10;
    }

    return result;
}

fn cleanup(mut number: Decimal) -> Decimal {
    let mut i = 0;
    while i < number.len() {
        if number[i] >= 10 {
            number[i] -= 10;
            if number.len() == i+1 {
                number.push(1);
            } else {
                number[i+1] += 1;
            }
        }
        i += 1;
    }

    return number;
}

fn add(number1: &Decimal, number2: &Decimal) -> Decimal {
    let mut result = vec![0; max(number1.len(), number2.len())];
    for i in 0..number1.len() {
        result[i] += number1[i];
    }
    for i in 0..number2.len() {
        result[i] += number2[i];
    }
    return cleanup(result);
}

fn shift(mut number: Decimal) -> Decimal {
    number.insert(0, 0);
    return number;
}

fn multiply(number1: &Decimal, number2: &Decimal) -> Decimal {
    let mut result = Vec::new();
    let mut multiplee = number2.clone();
    for i in 0..number1.len() {
        for _ in 0..number1[i] {
            result = add(&result, &multiplee);
        }
        multiplee = shift(multiplee);
    }
    return result;
}

fn geq(number1: &Decimal, number2: &Decimal) -> bool {
    if number1.len() > number2.len() {
        return true;
    }
    if number1.len() < number2.len() {
        return false;
    }

    for i in 1..=number1.len() {
        let i = number1.len() - i;

        if number1[i] > number2[i] {
            return true;
        }
    }

    return true;
}

fn tick() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let width = iter.next().unwrap();
    let height = iter.next().unwrap();

    let mut board = Vec::new();
    let mut negatives = HashSet::new();
    for i in 0..height {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let row = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let mut inserting_row = Vec::new();
        for j in 0..row.len() {
            let number = decimalify(row[j].abs());
            inserting_row.push(number);

            if row[j] < 0 {
                negatives.insert((i, j));
            }
        }

        board.push(inserting_row);
    }

    let mut max_column = 0;
    let mut max_value = Vec::new();
    let mut max_negative = false;
    for i in 0..width {
        let mut result = vec![1];
        let mut negative = false;
        for j in 0..height {
            result = multiply(&result, &board[j][i]);
            if negatives.contains(&(j, i)) {
                negative = !negative;
            }
        }

        // if result >= max_value
        if !negative && max_negative
                || !negative && !max_negative && geq(&result, &max_value)
                || negative && max_negative && geq(&max_value, &result) {
            max_value = result;
            max_column = i;
            max_negative = negative;
        }
    }

    println!("{}", max_column + 1);
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let testcase_count = input.trim().parse::<usize>().unwrap();

    for _ in 0..testcase_count {
        tick();
    }
}
