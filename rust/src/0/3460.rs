use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let count = input.trim().parse::<u32>().unwrap();
    for _ in 0..count {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let n = input.trim().parse::<u32>().unwrap();

        for i in 0..20 {
            if n & (1 << i) != 0 {
                print!("{} ", i);
            }
        }
        println!();
    }
}
