use std::io::stdin;

fn tick() {
    let count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    let mut score = 0;
    for _ in 0..count {
        let (a, b) = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let mut iter = input.trim().split_whitespace();
            (
                iter.next().unwrap().to_string(),
                iter.next().unwrap().to_string(),
            )
        };

        if a == "R" && b == "P" || a == "P" && b == "S" || a == "S" && b == "R" {
            score += 1;
        } else if a == b {
            score += 0;
        } else {
            score -= 1;
        }
    }

    if score > 0 {
        println!("Player 2");
    } else if score < 0 {
        println!("Player 1");
    } else {
        println!("TIE");
    }
}

fn main() {
    let cases = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    for _ in 0..cases {
        tick();
    }
}
