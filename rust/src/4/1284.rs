use std::io::stdin;

fn get_width(n: i32) -> i32 {
    match n {
        0 => 4,
        1 => 2,
        _ => 3,
    }
}

fn main() {
    let mut input = String::new();

    loop {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let mut n: i32 = input.trim().parse().unwrap();

        if n == 0 {
            break;
        }

        let mut width = 1;
        while n > 0 {
            width += get_width(n % 10) + 1;
            n /= 10;
        }

        println!("{}", width);
    }
}
