use std::io::stdin;

fn gcd(a: u64, b: u64) -> u64 {
    if a < b {
        return gcd(b, a);
    }

    if b == 0 {
        return a;
    }

    return gcd(b, a%b);
}

fn main() {
    // --- get number
    let number = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u64>().unwrap()
    };

    // --- get gcd of all characters
    let lcm = {
        // get all numbers
        let mut numbers = Vec::new();

        let mut number = number;
        while number > 0 {
            let n = number % 10;
            number /= 10;

            if n <= 1 {
                continue;
            }
            if numbers.contains(&n) {
                continue;
            }

            numbers.push(n);
        }

        // calculate gcd
        let mut lcm = 1;

        for number in numbers {
            lcm = number * lcm / gcd(number, lcm);
        }

        lcm
    };

    // --- find the number
    let result = {
        let mut exponential = 0;
        loop {
            let invoker = number * 10u64.pow(exponential) / lcm * lcm;
            let min_inclusive = number * 10u64.pow(exponential);
            let max_exclusive = (number + 1) * 10u64.pow(exponential);

            let mut i = 0;
            while invoker + i * lcm < min_inclusive {
                i += 1;
            }

            if invoker + i * lcm >= max_exclusive {
                exponential += 1;
                continue;
            }

            break invoker + i * lcm;
        }
    };

    println!("{}", result);
}
