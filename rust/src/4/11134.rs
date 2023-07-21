use std::io::stdin;

fn tick() {
    let (cookies, eats) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input.trim().split_whitespace();

        let cookies = iter.next().unwrap().parse::<f64>().unwrap();
        let eats = iter.next().unwrap().parse::<f64>().unwrap();

        (cookies, eats)
    };

    println!("{}", (cookies / eats).ceil() as u32);
}

fn main() {
    let cases = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    for _ in 0..cases {
        tick();
    }
}
