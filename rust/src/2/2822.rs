use std::io::stdin;

fn main() {
    let mut scores = Vec::new();

    for _ in 0..8 {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let score = input.trim().parse::<u32>().unwrap();
        scores.push(score);
    }

    let mut sorted_scores = scores.clone();
    sorted_scores.sort();

    let mut indexes = Vec::new();
    let mut result = 0;

    for i in 3..8 {
        let score = sorted_scores[i];
        let index = scores.iter().position(|&x| x == sorted_scores[i]).unwrap();

        indexes.push(index + 1);
        result += score;
    }

    indexes.sort();
    println!("{}", result);
    for index in indexes.iter() {
        print!("{} ", index);
    }
}
