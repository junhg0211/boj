use std::io::stdin;
use std::cmp::max;

fn main() {
    let (count, fee, price_pu) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap());

        (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    };

    let mut woods = Vec::new();
    let mut max_length = 0;
    for _ in 0..count {
        let length = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            input.trim().parse::<i64>().unwrap()
        };

        woods.push(length);

        max_length = max(max_length, length);
    }

    let mut max_price = i64::MIN;
    for length in 1..=max_length {
        let mut price = 0;

        for wood in woods.iter() {
            let parts = wood / length;
            let cuts = if wood % length == 0 { wood / length - 1 } else { wood / length };

            let this_price = parts * length * price_pu - cuts * fee;

            if this_price > 0 {
                price += this_price;
            }
        }

        max_price = max(max_price, price);
    }

    println!("{}", max_price);
}
