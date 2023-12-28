use std::io::stdin;
use std::cmp::min;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());

    let mut girls = iter.next().unwrap();
    let mut boys = iter.next().unwrap();
    let mut internships = iter.next().unwrap();

    while internships > 0 {
        if boys*2-1 <= girls {
            girls -= 1;
            internships -= 1;
        } else {
            boys -= 1;
            internships -= 1;
        }
    }

    println!("{}", min(girls / 2, boys));
}
