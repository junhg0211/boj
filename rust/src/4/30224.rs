use std::io::stdin;

fn contains_7(mut number: u32) -> bool {
    while number > 0 {
        if number % 10 == 7 {
            return true;
        }

        number /= 10;
    }

    return false;
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let number = input.trim().parse::<u32>().unwrap();

    let divisible = number % 7 == 0;
    let contains = contains_7(number);

    // println!("{} {}", divisible, contains);

    if !contains && !divisible {
        println!("0");
    } else if !contains && divisible {
        println!("1");
    } else if contains && !divisible {
        println!("2");
    } else {
        println!("3");
    }
}
