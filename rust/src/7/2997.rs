use std::io::stdin;

fn main() {
    let mut numbers = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    };
    numbers.sort();

    let difference = vec![numbers[1] - numbers[0], numbers[2] - numbers[1]];

    if difference[0] == difference[1] {
        println!("{}", numbers[2] + difference[0]);
    } else if difference[0] > difference[1] {
        println!("{}", (numbers[0] + numbers[1]) / 2);
    } else {
        println!("{}", (numbers[2] + numbers[1]) / 2);
    }
}
