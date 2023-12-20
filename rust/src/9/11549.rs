use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let answer = input.trim().parse::<u32>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());

    let mut count = 0;
    for _ in 0..5 {
        let guess = iter.next().unwrap();

        if guess == answer {
            count += 1;
        }
    }

    println!("{}", count);
}
