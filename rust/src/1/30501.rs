use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<usize>().unwrap();

    let mut the_name = String::new();
    for _ in 0..count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let string = input.trim().to_string();
        let chars = string.chars().collect::<Vec<_>>();

        for i in 0..string.len() {
            if chars[i] == 'S' {
                the_name = string;
                break;
            }
        }
    }

    println!("{}", the_name);
}
