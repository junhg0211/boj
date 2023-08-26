use std::io::stdin;

fn main() {
    let mut max = 0;
    let mut row = 0;
    let mut column = 0;

    for i in 0..9 {
        let numbers = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            input
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        };

        for j in 0..9 {
            let number = numbers[j];

            if number >= max {
                max = number;
                row = i;
                column = j;
            }
        }
    }

    println!("{}", max);
    println!("{} {}", row + 1, column + 1);
}
