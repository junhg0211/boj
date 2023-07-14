use std::io::stdin;

fn main() {
    // get a, b, c sorted
    let (a, b, c) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut v: Vec<u32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        v.sort();

        (v[0], v[1], v[2])
    };

    // get order
    let order = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    };

    // print accourdingly
    for now in order.chars() {
        print!("{} ", match now {
            'A' => a,
            'B' => b,
            'C' => c,
            _ => panic!("Invalid order"),
        });
    }
}
