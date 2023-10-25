use std::io::stdin;

fn main() {
    let (a, b) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap());

        (iter.next().unwrap(), iter.next().unwrap())
    };

    let (k, x) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap());

        (iter.next().unwrap(), iter.next().unwrap())
    };

    let mut count = 0;
    for i in a..=b {
        if (i-k).abs() <= x {
            count += 1;
        }
    }

    match count {
        0 => println!("IMPOSSIBLE"),
        s => println!("{}", s),
    };
}
