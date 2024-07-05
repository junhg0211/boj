use std::io::stdin;

fn main() {
    let mut input = String::new();
    loop {
        input.clear();
        let bytes = stdin().read_line(&mut input).unwrap();

        if bytes == 0 {
            break;
        }

        let mut string = input.trim().to_string();
        loop {
            let new = string.replace("BUG", "");

            if new == string {
                break;
            }

            string = new;
        }

        println!("{}", string);
    }
}
