use std::io::stdin;

fn main() {
    let count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<i32>().unwrap()
    };

    for _ in 0..count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let number = input.trim().parse::<i32>().unwrap();

        println!("{} is {}", number, match number.abs() % 2 {
            0 => "even",
            1 => "odd",
            _ => "",
        });
    }
}
