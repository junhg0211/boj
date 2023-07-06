use std::io::stdin;

fn get_char(n: i32, x: i32, y: i32) -> char {
    let x = x % (2*n - 1);
    let y = y % (2*n - 1);
    let d = (n-1-x).abs() + (n-1-y).abs();

    match d >= n {
        true => '.',
        false => char::from_u32('a' as u32 + d as u32 % 26).unwrap(),
    }
}

fn main() {
    // get numbers
    let (n, r1, c1, r2, c2) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let numbers = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        (numbers[0], numbers[1], numbers[2], numbers[3], numbers[4])
    };

    // print result
    for i in r1..=r2 {
        for j in c1..=c2 {
            print!("{}", get_char(n, i, j));
        }
        println!();
    }
}
