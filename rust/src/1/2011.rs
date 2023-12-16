use std::io::stdin;

const DIVISOR: u32 = 1000000;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let password = input
        .trim()
        .chars()
        .map(|x| x as u32 - '0' as u32)
        .collect::<Vec<_>>();

    let mut now = (0, 1);  // with, without
    let mut previous = 0;
    for &character in password.iter() {
        let o = if character != 0 { now.0 + now.1 } else { 0 };
        let long = previous * 10 + character;
        let i = if 10 <= long && long <= 26 { now.1 } else { 0 };

        now = (i % DIVISOR, o % DIVISOR);
        previous = character;
    }

    println!("{}", (now.0 + now.1) % DIVISOR);
}
