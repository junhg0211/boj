use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap());

    let m = iter.next().unwrap();
    let k = iter.next().unwrap();

    if m % k == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
