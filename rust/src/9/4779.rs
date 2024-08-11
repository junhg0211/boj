use std::io::stdin;

fn draw_line(length: u32) {
    if length == 1 {
        print!("-");
        return;
    }

    draw_line(length / 3);

    for _ in 0..length / 3 {
        print!(" ");
    }

    draw_line(length / 3);
}

fn main() {
    loop {
        let mut input = String::new();
        let bytes = stdin().read_line(&mut input).unwrap();

        if bytes == 0 {
            break;
        }

        let n = input.trim().parse::<u32>().unwrap();
        let length = u32::pow(3, n);

        draw_line(length);
        println!();
    }
}
