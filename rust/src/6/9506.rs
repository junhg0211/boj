use std::io::stdin;

fn get_divisors(number: i32) -> Vec<i32> {
    let mut result = Vec::new();
    let mut stack = Vec::new();

    for i in 1..=(number as f64).sqrt() as i32 {
        if number % i != 0 {
            continue;
        }

        if number / i == i {
            result.push(i);
            break;
        }

        result.push(i);
        stack.push(number / i);
    }

    loop {
        match stack.pop() {
            Some(thing) => result.push(thing),
            None => break,
        }
    }

    return result;
}

fn main() {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let number = input.trim().parse::<i32>().unwrap();

        if number == -1 {
            break;
        }

        let divisors = get_divisors(number);
        if divisors.iter().sum::<i32>() != number * 2 {
            println!("{} is NOT perfect.", number);
            continue;
        }

        print!("{} = ", number);
        for (i, divisor) in divisors.iter().enumerate() {
            if i == divisors.len() - 1 {
                break;
            }

            print!("{}", divisor);

            if i < divisors.len() - 2 {
                print!(" + ");
            }
        }
        println!();
    }
}
