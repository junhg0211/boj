use std::io::stdin;

fn main() {
    let from = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<i32>().unwrap()
    };

    let to = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<i32>().unwrap()
    };

    println!("{}", (to - from + 24) % 24);
}
