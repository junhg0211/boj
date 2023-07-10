use std::io::stdin;

fn get_direction(direction: u32) -> char {
    match direction {
        0 => 'N',
        1 => 'E',
        2 => 'S',
        3 => 'W',
        _ => panic!("Invalid direction"),
    }
}

fn main() {
    // init direction
    let mut direction: u32 = 0;

    // handle instructions
    for _ in 0..10 {
        // get instruction
        let instruction = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            input.trim().parse::<u32>().unwrap()
        };

        // change direction by instruction
        direction = match instruction {
            1 => (direction + 1) % 4,
            3 => (direction + 3) % 4,
            2 => (direction + 2) % 4,
            _ => panic!("Invalid instruction"),
        }
    }

    // print current direction
    println!("{}", get_direction(direction));
}
