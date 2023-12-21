use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let a = input.trim().parse::<u32>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let b = input.trim().parse::<u32>().unwrap();

    if a <= 50 && b <= 10 {
        println!("White");
    } else if b > 30 {
        println!("Red");
    } else {
        println!("Yellow");
    }
}
