use std::io::stdin;

fn tick() {
    let mut characters = 0;
    let mut errors = 0;
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let length = input.trim().len();

        if length == 0 {
            break;
        }

        characters += input.trim().len();
        for character in input.trim().chars() {
            if character == '#' {
                errors += 1;
            }
        }
    }

    let ratio = (1000.0 * (characters as f64 - errors as f64) / characters as f64).round() / 10.0;

    if (ratio * 10.0) as i32 % 10 == 0 {
        println!("Efficiency ratio is {}%.", ratio as i32);
    } else {
        println!("Efficiency ratio is {}%.", ratio);
    }
}

fn main() {
    let count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    for _ in 0..count {
        tick();
    }
}