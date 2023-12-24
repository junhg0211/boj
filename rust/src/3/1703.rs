use std::io::stdin;

fn main() {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap());

        let count = iter.next().unwrap();

        if count == 0 {
            break;
        }

        let mut leaf = 1;
        for _ in 0..count {
            let multiplier = iter.next().unwrap();
            let subtractor = iter.next().unwrap();

            leaf *= multiplier;
            leaf -= subtractor;
        }

        println!("{}", leaf);
    }
}
