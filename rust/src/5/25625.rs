use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    let duration = iter.next().unwrap();
    let delta = iter.next().unwrap();

    if delta < duration {
        println!("{}", duration + delta);
        return;
    }

    if delta == duration {
        println!("{}", duration * 2);
        return;
    }

    println!("{}", delta - duration);
}
