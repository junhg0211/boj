use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let bagunee_count = iter.next().unwrap();
    let reverse_count = iter.next().unwrap();

    let mut bagunees = (1..=bagunee_count).collect::<Vec<_>>();

    for _ in 0..reverse_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());

        let start = iter.next().unwrap() - 1;
        let end = iter.next().unwrap() - 1;

        let mut stack = Vec::new();

        for i in start..=end {
            stack.push(bagunees[i]);
        }

        let mut i = 0;
        loop {
            match stack.pop() {
                Some(thing) => {
                    bagunees[start+i] = thing;
                    i += 1;
                },
                None => break,
            }
        }
    }

    for bagunee in &bagunees {
        print!("{} ", bagunee);
    }
    println!();
}
