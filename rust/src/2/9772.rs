use std::io::stdin;

fn main() {
    loop {
        // get coördination
        let (x, y) = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let numbers = input.trim()
                .split_whitespace()
                .map(|x| x.parse::<f64>().unwrap())
                .collect::<Vec<f64>>();

            (numbers[0], numbers[1])
        };

        // print quaternion
        if x == 0.0 || y == 0.0 {
            println!("AXIS");
        } else if x > 0.0 && y > 0.0 {
            println!("Q1");
        } else if x < 0.0 && y > 0.0 {
            println!("Q2");
        } else if x < 0.0 && y < 0.0 {
            println!("Q3");
        } else if x > 0.0 && y < 0.0 {
            println!("Q4");
        }

        // terminal condition
        if x == 0.0 && y == 0.0 {
            break;
        }
    }
}
