use std::io::stdin;

fn get_value(previous: u32, char: char) -> u32 {
    match char {
        'S' => 10,
        'P' => 10 - previous,
        '-' => 0,
        _ => char as u32 - '0' as u32,
    }
}

fn eval(pins: &String) -> u32 {
    let mut result = 0;

    let mut previous = 0;
    for char in pins.chars() {
        previous = get_value(previous, char);

        result += previous;
    }

    result
}

fn main() {
    // get pins per frame
    let frames = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut pins = Vec::<String>::new();
        let mut op = 0;

        for char in input.trim().chars() {
            if op == 0 {
                pins.push(String::new());
            }

            let this = pins.last_mut().unwrap();
            this.push(char);

            if pins.len() < 10 && (char == 'S' || op == 1) {
                op = 0;
                continue
            }

            op += 1;
        }

        pins
    };

    // calculate points
    let mut points = [0 as u32; 10];
    for i in 0..10 {
        let i = 9 - i;

        points[i] = match i {
            9 => eval(&frames[i]),
            _ => {
                if frames[i] == "S" {
                    let previous = get_value(0, frames[i + 1].chars().nth(0).unwrap());
                    let next = if previous == 10 && i != 8 {
                        get_value(previous, frames[i + 2].chars().nth(0).unwrap())
                    } else {
                        get_value(previous, frames[i + 1].chars().nth(1).unwrap())
                    };
                    10 + previous + next
                } else if frames[i].chars().nth(1).unwrap() == 'P' {
                    10 + get_value(0, frames[i + 1].chars().nth(0).unwrap())
                } else {
                    eval(&frames[i])
                }
            },
        };

        // println!("{} {} {}", i, frames[i], eval(&frames[i]));
    }

    // get result
    let result = {
        let mut sum = 0;
        for point in points.iter() {
            sum += point;
        }
        sum
    };
    println!("{}", result);
}