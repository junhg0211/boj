use std::io::stdin;

#[test]
fn test_1() {
    assert_eq!(tick(vec![5, 4, 3, 2, 1]), 38);
}
#[test]
fn test_2() {
    assert_eq!(tick(vec![5, 3, 2, 6]), 90);
}
#[test]
fn test_3() {
    assert_eq!(tick(vec![1, 2, 1]), 2);
}
#[test]
fn test_4() {
    assert_eq!(tick(vec![2, 1, 2]), 4);
}
#[test]
fn test_5() {
    assert_eq!(tick(vec![3, 2, 100, 2, 1]), 406);
}

fn tick(mut numbers: Vec<u32>) -> u32 {
    let mut result = 0;
    while numbers.len() >= 3 {
        let mut min = u32::MAX;
        let mut min_i = 0;
        for i in 1..numbers.len() - 1 {
            let prod = numbers[i - 1] * numbers[i] * numbers[i + 1];
            if prod < min {
                min = prod;
                min_i = i;
            }
        }

        result += min;
        // println!("{}", min);

        let mut new_numbers = Vec::new();
        for i in 0..min_i {
            new_numbers.push(numbers[i]);
        }
        for i in min_i + 1..numbers.len() {
            new_numbers.push(numbers[i]);
        }

        numbers = new_numbers;
    }

    result
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<usize>().unwrap();

    let mut numbers = Vec::new();
    for i in 0..count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap());

        let number = iter.next().unwrap();
        numbers.push(number);

        if i + 1 == count {
            numbers.push(iter.next().unwrap());
        }
    }

    println!("{}", tick(numbers));
}
