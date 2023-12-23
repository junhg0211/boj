use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let a = input.trim().parse::<u32>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let b = input.trim().parse::<u32>().unwrap();

    let shin = (a + b) % 12;

    if shin == 0 {
        println!("12");
    } else {
        println!("{}", shin);
    }
}
