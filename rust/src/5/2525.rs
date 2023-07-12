use std::io::stdin;

fn main() {
    // get numbers
    let (mut hour, mut minute) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let numbers = input
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        (numbers[0], numbers[1])
    };

    let minutes = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().parse::<i32>().unwrap()
    };

    // calculate
    minute += minutes;

    // carry
    while minute >= 60 {
        minute -= 60;
        hour += 1;
    }

    while hour >= 24 {
        hour -= 24;
    }

    // print
    println!("{} {}", hour, minute);
}
