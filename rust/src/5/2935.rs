use std::io::stdin;
use std::cmp::max;

fn get_string() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    // get numbers
    let number1 = get_string();
    let operation = get_string();
    let number2 = get_string();

    // print result by operation
    if operation == "*" {
        print!("1");
        for _ in 0..number1.len() + number2.len() - 2 {
            print!("0");
        }
        println!();
    } else {
        let mut result = String::new();

        let length = max(number1.len(), number2.len());
        for i in 0..length as i32 {
            let c1 = match number1.chars().nth((number1.len() as i32 - i - 1) as usize) {
                Some(c) => c,
                None => '0',
            };
            let c2 = match number2.chars().nth((number2.len() as i32 - i - 1) as usize) {
                Some(c) => c,
                None => '0',
            };

            if c1 == '0' && c2 == '0' {
                result.insert_str(0, "0");
            } else if c1 == '1' && c2 == '1' {
                result.insert_str(0, "2");
            } else {
                result.insert_str(0, "1");
            }
        }

        println!("{}", result);
    }
}
