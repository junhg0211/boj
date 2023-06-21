use std::io::stdin;

fn t(n: u32) -> u32 {
    n * (n + 1) / 2
}

fn main() {
    // get count
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count: u32 = input
        .trim()
        .parse()
        .unwrap();

    for _ in 0..count {
        // get n
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let n = input
            .trim()
            .parse()
            .unwrap();

        // calculate result
        let mut result = 0;
        for k in 1..=n {
            result += k * t(k + 1);
        }

        // print result
        println!("{}", result);
    }
}
