use std::io::stdin;
use std::cmp::min;

fn main() {
    let chickens = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    let (coke, beer) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input.trim().split_whitespace();
        let coke = iter.next().unwrap().parse::<u32>().unwrap();
        let beer = iter.next().unwrap().parse::<u32>().unwrap();

        (coke, beer)
    };

    println!("{}", min(coke/2 + beer, chickens));
}
