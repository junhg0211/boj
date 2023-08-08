use std:io::stdin;
use std::collections::HashMap;

fn main() {
    let (n, m, q) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input.split_whitespace();

        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };

    let mut row_delta = HashMap::new();
    let mut col_delta = HashMap::new();

    for _ in 0..q {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input.split_whitespace();

        let (x, y, z) = (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap() - 1,
            iter.next().unwrap().parse::<i32>().unwrap(),
        );

        if x == 1 {
            *row_delta.entry(y).or_insert(0) += z;
        } else {
            *col_delta.entry(y).or_insert(0) += z;
        }
    }

    // println!("{:?} {:?}", row_delta, col_delta);

    for i in 0..n {
        for j in 0..m {
            let mut value = 0;

            if row_delta.contains_key(&i) {
                value += row_delta[&i];
            }

            if col_delta.contains_key(&j) {
                value += col_delta[&j];
            }

            print!("{} ", value);
        }
        println!();
    }
}
