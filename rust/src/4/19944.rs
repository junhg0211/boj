use std::io::stdin;

fn main() {
    let (n, m) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input.split_whitespace();

        let n: u32 = iter.next().unwrap().parse().unwrap();
        let m: u32 = iter.next().unwrap().parse().unwrap();

        (n, m)
    };

    if m == 1 || m == 2 {
        println!("NEWBIE!");
    } else if m <= n {
        println!("OLDBIE!");
    } else {
        println!("TLE!");
    }
}
