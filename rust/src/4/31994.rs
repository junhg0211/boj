use std::io::stdin;

fn main() {
    let mut max_name = String::new();
    let mut max_count = 0;

    for _ in 0..7 {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();

        let name = iter.next().unwrap();
        let count = iter.next().unwrap().parse::<u32>().unwrap();

        if count > max_count {
            max_name = name.to_string();
            max_count = count;
        }
    }

    println!("{}", max_name);
}
