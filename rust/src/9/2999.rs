use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let chars = input.trim().chars().collect::<Vec<_>>();

    let length = chars.len();
    let mut r = 0;
    let mut c = 0;
    for i in 1..=length {
        if length % i != 0 {
            continue;
        }

        if i <= length / i && r < i {
            (r, c) = (i, length / i);
        }
    }

    for i in 0..length {
        let column = i / c;
        let row = i % c;

        print!("{}", chars[row * r + column]);
    }
    println!();
}
