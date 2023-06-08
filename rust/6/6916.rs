use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<u8>().unwrap();

    match input {
        0 => {
            println!(" * * *");
            println!("*     *");
            println!("*     *");
            println!("*     *");
            println!();
            println!("*     *");
            println!("*     *");
            println!("*     *");
            println!(" * * *");
        },
        1 => {
            println!();
            println!("      *");
            println!("      *");
            println!("      *");
            println!();
            println!("      *");
            println!("      *");
            println!("      *");
            println!();
        },
        2 => {
            println!(" * * *");
            println!("      *");
            println!("      *");
            println!("      *");
            println!(" * * *");
            println!("*");
            println!("*");
            println!("*");
            println!(" * * *");
        },
        3 => {
            println!(" * * *");
            println!("      *");
            println!("      *");
            println!("      *");
            println!(" * * *");
            println!("      *");
            println!("      *");
            println!("      *");
            println!(" * * *");
        },
        4 => {
            println!();
            println!("*     *");
            println!("*     *");
            println!("*     *");
            println!(" * * *");
            println!("      *");
            println!("      *");
            println!("      *");
            println!();
        },
        5 => {
            println!(" * * *");
            println!("*");
            println!("*");
            println!("*");
            println!(" * * *");
            println!("      *");
            println!("      *");
            println!("      *");
            println!(" * * *");
        },
        6 => {
            println!(" * * *");
            println!("*");
            println!("*");
            println!("*");
            println!(" * * *");
            println!("*     *");
            println!("*     *");
            println!("*     *");
            println!(" * * *");
        },
        7 => {
            println!(" * * *");
            println!("      *");
            println!("      *");
            println!("      *");
            println!();
            println!("      *");
            println!("      *");
            println!("      *");
            println!();
        },
        8 => {
            println!(" * * *");
            println!("*     *");
            println!("*     *");
            println!("*     *");
            println!(" * * *");
            println!("*     *");
            println!("*     *");
            println!("*     *");
            println!(" * * *");
        },
        9 => {
            println!(" * * *");
            println!("*     *");
            println!("*     *");
            println!("*     *");
            println!(" * * *");
            println!("      *");
            println!("      *");
            println!("      *");
            println!(" * * *");
        },
        _ => ()
    }
}
