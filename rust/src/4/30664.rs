use std::io::stdin;

fn main() {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let number = input.trim().chars();

        if input.trim() == "0" {
            return;
        }

        let mut now = 0;
        for letter in number {
            now *= 10;
            now += letter as u32 - '0' as u32;
            now %= 42;
        }

        if now == 0 {
            println!("PREMIADO");
        } else {
            println!("TENTE NOVAMENTE");
        }
    }
}
