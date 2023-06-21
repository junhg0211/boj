use std::io::stdin;

fn main() {
    // get line count
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count: i32 = input.trim().parse().unwrap();

    for i in 1..(count + 1) {
        println!("Hello World, Judge {}!", i);
    }
}
