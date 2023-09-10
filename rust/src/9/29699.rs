use std::io::stdin;

fn main() {
    let length = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<usize>().unwrap()
    };

    println!("{}", "WelcomeToSMUPC".chars().nth((length - 1) % 14).unwrap());
}
