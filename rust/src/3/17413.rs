use std::io::stdin;

fn print(chars: &Vec<char>) {
    for i in 0..chars.len() {
        print!("{}", chars[chars.len() - i - 1]);
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let chars = input.trim().chars();

    let mut buffer = Vec::new();
    let mut tag = false;
    for char in chars {
        if char == '>' {
            print!(">");
            tag = false;

            continue;
        }

        if tag {
            print!("{}", char);

            continue;
        }

        if char == ' ' {
            print(&buffer);
            buffer.clear();
            print!(" ");

            continue;
        }

        if char == '<' {
            print(&buffer);
            buffer.clear();
            print!("<");

            tag = true;
            continue;
        }

        buffer.push(char);
    }

    print(&buffer);
    println!();
}
