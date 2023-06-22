use std::{
    io::stdin,
    cmp::max,
};

fn main() {
    // get count
    let mut input = String::new();

    let count = {
        stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };

    // iterate the tries
    let mut max_score = 0;
    for _ in 0..count {
        let dice: Vec<u32> = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            input.trim().split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        };

        if dice[0] == dice[1] && dice[1] == dice[2] {
            max_score = max(max_score, 10000 + dice[0] * 1000);
        } else if dice[0] == dice[1] {
            max_score = max(max_score, 1000 + dice[0] * 100);
        } else if dice[0] == dice[2] {
            max_score = max(max_score, 1000 + dice[0] * 100);
        } else if dice[1] == dice[2] {
            max_score = max(max_score, 1000 + dice[1] * 100);
        } else {
            max_score = max(max_score, *dice.iter().max().unwrap() * 100);
        }
    }

    // print result
    println!("{}", max_score);
}
