use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<usize>().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut good_streak = 2;
    let mut result = 0;
    for i in 0..count - 2 {
        let a = numbers[i + 0];
        let b = numbers[i + 1];
        let c = numbers[i + 2];

        if !(a <= b && b <= c || a >= b && b >= c) {
            good_streak += 1;
        } else {
            good_streak = 2;
        }

        result = usize::max(result, good_streak);
    }

    println!("{}", result);
}
