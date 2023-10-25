use std::io::stdin;

fn main() {
    let count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    let goods = vec![
        "Never gonna give you up",
        "Never gonna let you down",
        "Never gonna run around and desert you",
        "Never gonna make you cry",
        "Never gonna say goodbye",
        "Never gonna tell a lie and hurt you",
        "Never gonna stop",
    ];

    let mut good = true;
    for _ in 0..count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if !goods.contains(&input.trim()) {
            good = false;
        }
    }

    match good {
        true => println!("No"),
        false => println!("Yes"),
    }
}
