use std::io::stdin;
use std::cmp::max;

#[derive(PartialEq)]
struct Permutation {
    with: Vec<u32>,
    subpermutation: Option<Box<Permutation>>,
    next_i: usize,
}

impl Permutation {
    fn new(with: Vec<u32>) -> Self {
        let mut result = Permutation {
            with: with,
            subpermutation: None,
            next_i: 0
        };

        if result.with.len() > 1 {
            result.next_i += 1;
            result.make_subpermutation();
        }

        return result;
    }

    fn make_subpermutation(&mut self) {
        let mut vector = self.with.clone();
        vector.remove(self.next_i-1);
        self.subpermutation = Some(Box::new(Permutation::new(vector)));
    }
}

impl Iterator for Permutation {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        // if length == 1
        if self.with.len() == 1 {
            self.next_i += 1;

            // if already have been
            if self.next_i > self.with.len() {
                return None;
            }

            // if first time calling next
            return Some(vec![self.with[self.next_i-1]]);
        }

        let next = self.subpermutation
            .as_mut().unwrap()
            .as_mut().next();

        // if subpermutation ended
        if next == None {
            self.next_i += 1;

            // if no more next prefix is available
            if self.next_i > self.with.len() {
                return None;
            }

            // make new subpermutation and return it
            self.make_subpermutation();
            return self.next();
        }

        // if next subpermutation is available
        let mut next = next.unwrap();
        let prefix = self.with[self.next_i-1];
        next.insert(0, prefix);
        return Some(next);
    }
}

fn calculate_sum(numbers: &Vec<u32>, start: usize, end: usize) -> u32 {
    let mut result = 0;
    for i in start..end {
        result += numbers[i % numbers.len()];
    }
    return result;
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let number_count = input.trim().parse::<usize>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut result = 0;
    for permutation in Permutation::new(numbers) {
        let mut lines = 0;

        let mut start = 0;
        let mut end = 0;

        while start < number_count {
            let sum = calculate_sum(&permutation, start, end);

            if sum < 50 {
                end += 1;
            } else if sum == 50 {
                lines += 1;
                end += 1;
            } else if sum > 50 {
                start += 1;
            }
        }

        result = max(result, lines);
    }

    println!("{}", result / 2);
}
