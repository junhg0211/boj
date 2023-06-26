use std::io::stdin;

fn main() {
    // get count
    let count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().parse::<u32>().unwrap()
    };

    // iterate
    for _ in 0..count {
        let (quantity, unit) = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let tokens: Vec<_> = input.trim().split_whitespace().collect();

            (tokens[0].parse::<f64>().unwrap(), tokens[1].to_owned())
        };

        // convert
        match unit.as_ref() {
            "kg" => {
                println!("{:.4} lb", quantity * 2.2046);
            },
            "l" => {
                println!("{:.4} g", quantity * 0.2642);
            },
            "lb" => {
                println!("{:.4} kg", quantity * 0.4536);
            },
            "g" => {
                println!("{:.4} l", quantity * 3.7854);
            },
            _ => ()
        }
    }
}
