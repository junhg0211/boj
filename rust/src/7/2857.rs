use std::io::stdin;

fn main() {
    let mut been = false;

    for i in 1..=5 {
        let name = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            input.trim().to_string()
        };

        if name.contains("FBI") {
            print!("{} ", i);
            been = true;
        }
    }

    if !been {
        println!("HE GOT AWAY!");
    }
}
