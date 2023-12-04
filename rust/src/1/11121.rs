use std::io::stdin;

fn tick() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace();

    let sended = iter.next().unwrap();
    let received = iter.next().unwrap();

    if sended == received {
        println!("OK");
    } else {
        println!("ERROR");
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<usize>().unwrap();

    for _ in 0..count {
        tick();
    }
}
