use std::io::stdin;

fn main() {
    // get count
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count: u32 = input.trim().parse().unwrap();

    for _ in 0..count {
        // get length
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let length: u32 = input.trim().parse().unwrap();

        println!("{}", "=".repeat(length as usize));
    }
}
