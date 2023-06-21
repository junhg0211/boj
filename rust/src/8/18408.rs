use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .unwrap();

    let mut ones = 0;
    let mut twos = 0;

    input
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .for_each(|x| match x {
            1 => ones += 1,
            2 => twos += 1,
            _ => (),
        });

    if ones > twos {
        println!("1");
    } else {
        println!("2");
    }
}
