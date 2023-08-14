use std::io::stdin;

fn tick() {
    let count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    let mut numbers = Vec::new();

    for _ in 0..count {
        let number = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            input.trim().parse::<u32>().unwrap()
        };

        numbers.push(number);
    }

    let mut result = 1;

    loop {
        let mut remainders = Vec::new();
        let mut good = true;

        for number in numbers.iter() {
            let remainder = number % result;

            if remainders.contains(&remainder) {
                good = false;
                break;
            }
            remainders.push(remainder);
        }

        if good {
            break;
        }

        result += 1;
    }

    println!("{}", result);
}

fn main() {
    let testcases = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    for _ in 0..testcases {
        tick();
    }
}
