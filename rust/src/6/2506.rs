use std::io::stdin;

fn main() {
    // get inputs
    let count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    let numbers = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
    };

    // calculate
    let mut result = 0;
    let mut previous_score = 0;

    for i in 0..count as usize {
        if numbers[i] == 1 {
            previous_score += 1;
        } else {
            previous_score = 0;
        }

        result += previous_score;
    }

    println!("{}", result);
}
