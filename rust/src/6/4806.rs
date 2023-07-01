use std::io::stdin;

fn main() {
    let mut lines = 0;

    loop {
        let mut input = String::new();
        let bytes = stdin().read_line(&mut input).unwrap();

        if bytes == 0 {
            break;
        } else {
            lines += 1;
        }
    }

    println!("{}", lines);
}
