use std::io::stdin;

fn is_prime(number: usize, sieve: &Vec<usize>) -> bool {
    for &prime in sieve.iter() {
        if number % prime == 0 {
            return false;
        }
    }

    return true;
}

fn extend_sieve_until(number: usize, sieve: &mut Vec<usize>, count: &mut Vec<usize>) {
    if sieve.is_empty() {
        sieve.push(2);
    }

    let mut i = sieve[sieve.len() - 1];
    while sieve[sieve.len() - 1] < number {
        if is_prime(i, sieve) {
            sieve.push(i);
            count[i] = count[i - 1] + 1;
        } else {
            count[i] = count[i - 1];
        }

        i += 1;
    }
}

fn main() {
    let mut sieve = Vec::new();
    let mut count = vec![1; 250000];

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let number = input.trim().parse::<usize>().unwrap();

        extend_sieve_until(number * 2, &mut sieve, &mut count);

        if number == 0 {
            return;
        }

        let count = count[number * 2 + 1] - count[number + 1];

        println!("{}", count);
    }
}
