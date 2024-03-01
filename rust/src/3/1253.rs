use std::io::stdin;
use std::collections::HashSet;

fn binary_search(on: &Vec<i32>, thing: i32) -> usize {
    let mut start = 0;
    let mut end = on.len();

    while start < end {
        let middle = (start + end) / 2;

        if on[middle] > thing {
            end = middle;
        } else if on[middle] < thing {
            start = middle;
        } else {
            return middle;
        }
    }

    return usize::MAX;
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let number_count = input.trim().parse::<usize>().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let mut numbers = input.trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    numbers.sort();

    let mut set = HashSet::new();
    for &number in &numbers {
        set.insert(number);
    }

    let mut result = 0;
    for i in 0..number_count {
        let target = numbers[i];
        for j in 0..number_count {
            if i == j {
                continue;
            }

            let now = numbers[j];

            if set.contains(&(target - now)) && target-now != target && target-now != now {
                result += 1;
                // println!("A {} {} {}", target, now, target-now);
                break;
            }

            if !set.contains(&(target - now)) {
                continue;
            }

            let index = binary_search(&numbers, target - now);

            if index != j && index != i {
                result += 1;
                // println!("B {} {} {}", target, now, numbers[index]);
                break;
            }

            if (index > 0 && numbers[index-1] == numbers[index] && index-1 != i && index-1 != j)
                || (index < numbers.len() - 1 && numbers[index+1] == numbers[index] && index+1 != i && index+1 != j) {
                result += 1;
                // println!("C {} {} {} {}", target, now, numbers[index-1], numbers[index+1]);
                break;
            }
        }
    }

    println!("{}", result);
}
