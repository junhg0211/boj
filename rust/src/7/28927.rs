use std::io::stdin;

fn get_minutes() -> u32 {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());

    let t = iter.next().unwrap();
    let e = iter.next().unwrap();
    let f = iter.next().unwrap();

    t * 3 + e * 20 + f * 120
}

fn main() {
    let max = get_minutes();
    let mel = get_minutes();

    if max > mel {
        println!("Max");
    } else if max < mel {
        println!("Mel");
    } else {
        println!("Draw");
    }
}
