use std::io::stdin;

fn main() {
    let mut length = 0;
    loop {
        let mut input = String::new();
        let bytes = stdin().read_line(&mut input).unwrap();

        if bytes == 0 {
            break;
        }

        let words = input.trim().split_whitespace();

        for word in words {
            if word == "<br>" {
                println!();
                length = 0;
                continue;
            }

            if word == "<hr>" {
                if length > 0 {
                    println!();
                    length = 0;
                }
                println!("--------------------------------------------------------------------------------");
                continue;
            }

            if length + word.len() > 80 {
                println!();
                length = 0;
            } else if length > 0 {
                print!(" ");
            }

            print!("{}", word);
            length += 1 + word.len();
        }
    }
    println!();
}
