use std::io::stdin;

fn main() {
    let mut diff = 0;
    for _ in 0..8 {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        for c in input.chars() {
            diff += match c {
                'K' => 0,
                'P' => 1,
                'N' => 3,
                'B' => 3,
                'R' => 5,
                'Q' => 9,
                'k' => -0,
                'p' => -1,
                'n' => -3,
                'b' => -3,
                'r' => -5,
                'q' => -9,
                _ => 0,
            };
        }
    }

    println!("{}", diff);
}
