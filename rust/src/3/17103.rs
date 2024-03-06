use std::io::stdin;

fn sqrt(number: u32) -> u32 {
    return (number as f64).sqrt() as u32;
}

fn is_prime(number: u32, sieve: &Vec<u32>) -> bool {
    for &prime in sieve.iter() {
        if prime > sqrt(number) {
            return true;
        }

        if number % prime == 0 {
            return false;
        }
    }

    return true;
}

fn extend_sieve(number: u32, sieve: &mut Vec<u32>) {
    if sieve.is_empty() {
        sieve.push(2);
    }

    let mut i = *sieve.last().unwrap() + 1;
    while i < number {
        if is_prime(i, sieve) {
            sieve.push(i);
        }

        i += 1;
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let testcase_count = input.trim().parse::<usize>().unwrap();

    let mut sieve = Vec::new();

    for _ in 0..testcase_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let number = input.trim().parse::<u32>().unwrap();

        extend_sieve(number, &mut sieve);
        // println!("{:?}", sieve);

        let mut result = 0;
        for &prime in sieve.iter() {
            if prime > number / 2 {
                break;
            }

            if let Ok(_) = sieve.binary_search(&(number - prime)) {
                result += 1;
                // println!("{} {}", prime, number - prime);
            }
        }

        println!("{}", result);
    }
}
