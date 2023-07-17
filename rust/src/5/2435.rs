use std::io::stdin;

fn main() {
    // get numbers
    let (days, strike) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input.trim().split_whitespace();

        (
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap()
        )
    };

    let temperatures = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    };

    // find max
    let mut max = i32::min_value();
    for i in 0..(days - strike + 1) {
        let mut sum = 0;

        for j in i..(i + strike) {
            sum += temperatures[j as usize];
        }

        if sum > max {
            max = sum;
            // println!("{} {} ", max, i);
        }
    }

    // print max
    println!("{}", max);
}
