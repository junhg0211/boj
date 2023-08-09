use std::io::stdin;

fn main() {
    let (x, k) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input.split_whitespace();

        (
            iter.next().unwrap().parse::<i32>().unwrap() * 1000,
            iter.next().unwrap().parse::<i32>().unwrap() * 1750,
        )
    };

    println!("{}", x/k * k);
}
