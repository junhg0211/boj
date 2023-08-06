use std::io::stdin;

fn main() {
    let (count, price) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let count: usize = iter.next().unwrap().parse().unwrap();
        let price: u32 = iter.next().unwrap().parse().unwrap();
        (count, price)
    };

    let prices: Vec<_> = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect()
    };

    let mut min = u32::MAX;
    for i in 0..count-1 {
        let the_price = prices[i] + prices[i+1];
        if the_price < min {
            min = the_price;
        }
    }

    println!("{}", min * price);
}
