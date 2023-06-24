use std::io::stdin;

fn main() {
    // get chanes
    let mut input = String::new();
    let changes = {
        stdin().read_line(&mut input).unwrap();
        input.trim().chars()
    };

    // calculate position
    let mut position = 0;
    for c in changes {
        match c {
            'A' => match position {
                0 => position = 1,
                1 => position = 0,
                _ => (),
            },
            'B' => match position {
                1 => position = 2,
                2 => position = 1,
                _ => (),
            },
            'C' => match position {
                0 => position = 2,
                2 => position = 0,
                _ => (),
            },
            _ => (),
        }
    }

    println!("{}", position + 1);
}
