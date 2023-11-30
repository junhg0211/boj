use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let now = input.trim().parse::<u32>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let weeks = input.trim().parse::<u32>().unwrap();

    if now + weeks * 7 <= 30 {
        println!("1");
    } else {
        println!("0");
    }
}
