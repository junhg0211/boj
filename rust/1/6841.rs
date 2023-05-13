use std::io;

fn main() {
    let mut input = String::new();

    loop {
        io::stdin().read_line(&mut input)
            .expect("error on readling stdin");
        input = input.trim().to_string();

        if input == "CU" {
            println!("see you");
        } else if input == ":-)" {
            println!("I’m happy");
        } else if input == ":-(" {
            println!("I’m unhappy");
        } else if input == ";-)" {
            println!("wink");
        } else if input == ":-P" {
            println!("stick out my tongue");
        } else if input == "(~.~)" {
            println!("sleepy");
        } else if input == "TA" {
            println!("totally awesome");
        } else if input == "CCC" {
            println!("Canadian Computing Competition");
        } else if input == "CUZ" {
            println!("because");
        } else if input == "TY" {
            println!("thank-you");
        } else if input == "YW" {
            println!("you're welcome");
        } else if input == "TTYL" {
            println!("talk to you later");
            break;
        } else {
            println!("{}", input);
        }

        input = "".to_string();
    }
}
