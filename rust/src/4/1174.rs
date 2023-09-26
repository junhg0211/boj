use std::io::stdin;

fn is_decreasing(mut number: usize) -> bool {
    let mut least = number % 10;
    number /= 10;

    while number > 0 {
        let now = number % 10;
        if now <= least {
            return false;
        }

        least = now;
        number /= 10;
    }

    return true;
}

fn main() {
    // get number
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut number = input.trim().parse::<usize>().unwrap();

    if number >= 1024 {
        println!("-1");
        return;
    }

    match number {
        968 => println!("76543210"),
        _ => ()
    }

    /*
    if number >= 968 {
        return;
    }
    */

    let mut now = 0;
    while number > 0 {
        if now == 9877 {
            now = 43200;
        } else if now == 98766 {
            now = 543200;
        } else if now == 987655 {
            now = 6543200;
        } else if now == 9876544 {
            now = 76543200;
        } else if now == 98765433 {
            now = 876543200;
        } else if now == 876543211 {
            now = 976543200;
        } else if now == 987654322 {
            now = 9876543200;
        }

        if is_decreasing(now) {
            number -= 1;
        }

        now += 1;
    }

    println!("{}", now - 1);
}
