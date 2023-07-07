use std::io::stdin;

fn main() {
    // get numbers
    let (mut l, mut r) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let numbers = input.trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        (numbers[0], numbers[1])
    };

    // calculate
    let mut result = 0;
    while l > 0 || r > 0 {
        if l % 10 == r % 10 {
            if r % 10 == 8 {
                result += 1;
            }
        } else {
            result = 0;
        }
        l /= 10;
        r /= 10;
    }

    // print
    println!("{}", result);
}
