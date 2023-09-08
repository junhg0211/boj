use std::io::stdin;

fn main() {
    let (_, baguni_width) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap());

        (iter.next().unwrap(), iter.next().unwrap())
    };

    let apple_count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    let mut moves = 0;

    let mut baguni_position = 1;
    for _ in 0..apple_count {
        let possible_right = baguni_position + baguni_width - 1;

        let apple_position = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            input.trim().parse::<u32>().unwrap()
        };

        if apple_position < baguni_position {
            moves += baguni_position - apple_position;
            baguni_position = apple_position;
        } else if possible_right < apple_position {
            moves += apple_position - possible_right;
            baguni_position += apple_position - possible_right;
        }
    }

    println!("{}", moves);
}
