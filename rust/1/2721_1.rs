use std::io::stdin;

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
        let n: f64 = input
            .trim()
            .parse()
            .unwrap();

        // calculate result
        let result = {
            let one = n * (n+1.0) / 2.0;
            let two = (2.0 * n + 1.0) * one / 2.0;
            let three = one.powf(2.0) / 2.0;
            one + two + three
        };

        // print result
        println!("{}", result);
    }
}
