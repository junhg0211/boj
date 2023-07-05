use std::io::stdin;

fn get_data(data: &String, i: i32, j: i32, width: i32) -> Option<char> {
    data.chars().nth((i * width + j) as usize)
}

#[derive(PartialEq)]
enum Direction {
    West, East, North, South
}

fn get_delta(direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::North => (0, -1),
        Direction::East => (1, 0),
        Direction::South => (0, 1),
        Direction::West => (-1, 0),
    }
}

fn tick() {
    let (height, width, data) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let tokens = input.trim().split_whitespace().collect::<Vec<_>>();

        (
            tokens[0].parse::<i32>().unwrap(),
            tokens[1].parse::<i32>().unwrap(),
            tokens[2].to_string(),
        )
    };

    let (mut x, mut y) = (0, 0);
    let mut direction = Direction::East;
    let mut rounds = 0;

    let mut buffer = String::new();
    let mut result = String::new();

    loop {
        // record character
        buffer.push(get_data(&data, y, x, width).unwrap());

        if buffer.len() >= 5 {
            // add char to result
            let c = ('A' as u8 + u8::from_str_radix(&buffer[..5], 2).unwrap() - 1) as char;
            if c == '@' {
                result.push(' ');
            } else {
                result.push(c);
            }

            // split buffer
            let tokens = buffer.split_at(5);
            buffer = tokens.1.to_string();
        }

        if (buffer.len() + result.len() * 5) as i32 >= width * height {
            println!("{}", result.trim());
            break;

        }

        // change direction
        direction = if direction == Direction::North && rounds + 1 == y {
            rounds += 1;
            Direction::East
        } else if direction == Direction::East && width - rounds - 1 == x {
            Direction::South
        } else if direction == Direction::South && height - rounds - 1 == y {
            Direction::West
        } else if direction == Direction::West && rounds == x {
            Direction::North
        } else {
            direction
        };

        // change position according to direction
        let (dx, dy) = get_delta(&direction);
        x += dx;
        y += dy;
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
