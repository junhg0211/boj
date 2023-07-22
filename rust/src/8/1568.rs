use std::io::stdin;

fn main() {
    let mut number = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    let mut flies = 1;
    let mut result = 0;

    while number > 0 {
        if number >= flies {
            number -= flies;
        } else {
            flies = 1;
            continue;
        }

        result += 1;
        flies += 1;
    }

    println!("{}", result);
}
