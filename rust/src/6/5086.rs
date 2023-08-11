use std::io::stdin;

fn main() {
    loop {
        let (a, b) = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let mut iter = input.trim().split_whitespace();

            (
                iter.next().unwrap().parse::<u32>().unwrap(),
                iter.next().unwrap().parse::<u32>().unwrap(),
            )
        };

        if a == 0 && b == 0 { break; }

        if b % a == 0 {
            println!("factor");
        } else if a % b == 0 {
            println!("multiple");
        } else {
            println!("neither");
        }
    }
}