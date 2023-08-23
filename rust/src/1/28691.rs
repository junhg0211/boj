use std::io::stdin;

fn main() {
    let c = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().to_owned()
    };

    match &c[..] {
        "M" => println!("MatKor"),
        "W" => println!("WiCys"),
        "C" => println!("CyKor"),
        "A" => println!("AlKor"),
        _ => println!("$clear"),
    }
}
