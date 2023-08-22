use std::io::stdin;

fn main() {
    let number = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let result = input.trim();

        &result.to_owned()[0..3]
    };

    match number {
        "555" => println!("YES"),
        _ => println!("NO"),
    };
}
