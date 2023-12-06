use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();

    let a = iter.next().unwrap().parse::<u32>().unwrap();
    let _operator = iter.next().unwrap();
    let b = iter.next().unwrap().parse::<u32>().unwrap();
    let _operator = iter.next().unwrap();
    let c = iter.next().unwrap().parse::<u32>().unwrap();

    if a + b == c {
        println!("YES");
    } else {
        println!("NO");
    }
}
