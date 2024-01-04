use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<usize>().unwrap();

    let mut min = u32::MAX;
    for _ in 0..count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap());
        let go = iter.next().unwrap();
        let bread = iter.next().unwrap();

        if go <= bread {
            if bread < min {
                min = bread;
            }
        }
    }

    if min == u32::MAX {
        println!("-1");
    } else {
        println!("{}", min);
    }
}
