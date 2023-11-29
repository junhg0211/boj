use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let length = input.trim().parse::<usize>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let string = input.trim().to_string();

    if string.chars().last() == Some('G') {
        println!("{}", &string[..string.len()-1]);
    } else {
        println!("{}G", string);
    }
}
