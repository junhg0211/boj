use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    let mut height = iter.next().unwrap();
    let mut width = iter.next().unwrap();

    let mut turns = 0;
    while height > 0 && width > 0 {
        if turns % 2 == 0 {
            height -= 1;
        } else {
            width -= 1;
        }
        turns += 1;
    }

    println!("{}", turns - 1);
}
