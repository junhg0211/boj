use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    let student_count = iter.next().unwrap();
    let subject_count = iter.next().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    for _ in 0..subject_count {
        let place = iter.next().unwrap();
        let percentage = place * 100 / student_count;

        if percentage <= 4 {
            print!("1 ");
        } else if percentage <= 11 {
            print!("2 ");
        } else if percentage <= 23 {
            print!("3 ");
        } else if percentage <= 40 {
            print!("4 ");
        } else if percentage <= 60 {
            print!("5 ");
        } else if percentage <= 77 {
            print!("6 ");
        } else if percentage <= 89 {
            print!("7 ");
        } else if percentage <= 96 {
            print!("8 ");
        } else {
            print!("9 ");
        }
    }
    println!();
}
