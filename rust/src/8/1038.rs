use std::io::stdin;

fn check(mut number: u64) -> bool {
    let mut last = 0;

    while number > 0 {
        let digit = number % 10 + 1;

        if digit <= last {
            return false;
        }

        last = digit;
        number /= 10;
    }

    return true;
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut index = input.trim().parse::<u64>().unwrap();

    let mut number = 0;
    loop {
        if !check(number) {
            number += 1;
            continue;
        }

        if index > 0 {
            index -= 1;
        } else {
            break;
        }

        number = match number {
            8765432 => 9543210,
            9876543 => 76543210,
            76543210 => 86543210,
            87654321 => 96543210,
            98765432 => 876543210,
            876543210 => 976543210,
            987654321 => 9876543210,
            9876543210 => {
                println!("-1");
                return;
            },
            _ => number + 1,
        };
    }

    println!("{}", number);
}
