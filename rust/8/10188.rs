use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count: i32 = input.trim().parse().unwrap();

    for _ in 0..count {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let numbers: Vec<_> = input.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

        let width = numbers[0];
        let height = numbers[1];

        for _ in 0..height {
            for _ in 0..width {
                print!("X");
            }
            println!();
        }
        println!();
    }
}
