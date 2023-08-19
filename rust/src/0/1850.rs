use std::io::stdin;

fn main() {
    let (mut a, mut b) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let numbers: Vec<_> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        (numbers[0], numbers[1])
    };

    if a > b {
        let c = b;
        b = a;
        a = c;
    }

    while a > 0 {
        (a, b) = (b%a, a);
    }

    let mut result = String::new();
    for _ in 0..b {
        result.push_str("1");
    }

    println!("{}", result);
}
