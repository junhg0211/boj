use std::io::stdin;

fn main() {
    let size = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().parse::<u64>().unwrap()
    };
    let numbers = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    };

    let one = numbers.iter().min().unwrap();

    let two = {
        let binding = [
            numbers[0] + numbers[1], numbers[1] + numbers[5], numbers[5] + numbers[4], numbers[4] + numbers[0],
            numbers[3] + numbers[0], numbers[3] + numbers[1], numbers[3] + numbers[5], numbers[3] + numbers[4],
            numbers[2] + numbers[0], numbers[2] + numbers[1], numbers[2] + numbers[5], numbers[2] + numbers[4]
        ];
        binding.iter().min().unwrap().to_owned()
    };

    let three = {
        let binding = [
            numbers[0] + numbers[1] + numbers[3], numbers[3] + numbers[1] + numbers[5],
            numbers[4] + numbers[5] + numbers[3], numbers[4] + numbers[3] + numbers[0],
            numbers[0] + numbers[1] + numbers[2], numbers[1] + numbers[5] + numbers[2],
            numbers[4] + numbers[5] + numbers[2], numbers[4] + numbers[0] + numbers[2]
        ];
        binding.iter().min().unwrap().to_owned()
    };

    let five = numbers.iter().sum::<u64>() - numbers.iter().max().unwrap();

    // print result
    if size == 1 {
        println!("{}", five);
        return;
    }

    println!("{}", one * (size-2) * (size-2)
             + one * (size-2) * (size-1) * 4
             + two * (size-1) * 4
             + two * (size-2) * 4
             + three * 4);
}
