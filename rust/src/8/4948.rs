use std::io::stdin;

fn sqrt(number: usize) -> usize {
    return (number as f64).sqrt() as usize;
}

fn is_prime(number: usize, sieve: &Vec<usize>) -> bool {
    for &prime in sieve.iter() {
        if number % prime == 0 {
            return false;
        }

        if sqrt(number) < prime {
            return true;
        }
    }

    return true;
}

fn extend_sieve_until(number: usize, sieve: &mut Vec<usize>) {
    if sieve.is_empty() {
        sieve.push(2);
    }

    let mut i = sieve[sieve.len() - 1];
    while sieve[sieve.len() - 1] < number {
        if is_prime(i, sieve) {
            sieve.push(i);
        }

        i += 1;
    }
}

fn main() {
    let mut sieve = Vec::new();

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let number = input.trim().parse::<usize>().unwrap();

        extend_sieve_until(number * 2, &mut sieve);

        if number == 0 {
            return;
        }

        let mut result = 0;
        for i in number + 1..=number * 2 {
            if let Ok(_) = sieve.binary_search(&i) {
                result += 1;
            }
        }

        println!("{}", result);
    }
}
