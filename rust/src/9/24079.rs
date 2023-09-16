use std::io::stdin;

fn main() {
    let x = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().parse::<u32>().unwrap()
    };
    let y = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().parse::<u32>().unwrap()
    };
    let z = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().parse::<u32>().unwrap()
    };

    match x+y <= z {
        true => println!("1"),
        false => println!("0"),
    };
}
