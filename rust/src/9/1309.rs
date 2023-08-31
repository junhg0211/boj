use std::io::stdin;

const RANGE: u32 = 9901;

fn main() {
    let height = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    let mut a = 1;  // possibilities when "  "
    let mut b = 1;  // possibilities when "L "
    let mut c = 1;  // possibilities when " L"

    for _ in 1..height {
        (a, b, c) = ((a+b+c) % RANGE, (a+b) % RANGE, (a+c) % RANGE);
    }

    println!("{}", (a + b + c) % RANGE);
}
