use std::io::stdin;

fn main() {
    // get n
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    // calculate
    println!("{}", {
        let nn1 = n * (n+1);

        let n2 = nn1 * (2*n + 1) / 6;
        let n1 = nn1 / 2;

        (n2 + n1) * 3 / 2
    });
}
