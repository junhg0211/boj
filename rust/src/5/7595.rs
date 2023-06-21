use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut count = input.trim().parse::<i32>().unwrap();

    while count != 0 {
        for i in 0..count {
            for _ in 0..(i + 1) {
                print!("*");
            }
            println!();
        }

        input.clear();
        stdin().read_line(&mut input).unwrap();
        count = input.trim().parse::<i32>().unwrap();
    }
}
