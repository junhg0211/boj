use std::io::stdin;

fn main() {
    let (first_cap, input_count, delete_count) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap());

        (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    };

    let mut cap = first_cap;
    let mut len = 0;

    for _ in 0..input_count + delete_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "0" => {
                len -= 1;
            },
            "1" => {
                len += 1;
                if len > cap {
                    cap *= 2;
                }
            },
            _ => (),
        }
    }

    println!("{}", cap);
}
