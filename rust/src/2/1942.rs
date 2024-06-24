use std::io::stdin;

fn tick() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();

    let binding = iter.next().unwrap().to_string();
    let mut start = binding
        .split(":")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let binding = iter.next().unwrap().to_string();
    let end = binding
        .split(":")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut count = 0;
    while start[0] != end[0] || start[1] != end[1] || start[2] != end[2] {
        let number = start[0] * 10000 + start[1] * 100 + start[2];
        if number % 3 == 0 {
            count += 1;
        }

        start[2] += 1;

        if start[2] >= 60 {
            start[2] -= 60;
            start[1] += 1;
        }

        if start[1] >= 60 {
            start[1] -= 60;
            start[0] += 1;
        }

        if start[0] >= 24 {
            start[0] -= 24;
        }
    }

    let number = start[0] * 10000 + start[1] * 100 + start[2];
    if number % 3 == 0 {
        count += 1;
    }

    println!("{}", count);
}

fn main() {
    for _ in 0..3 {
        tick();
    }
}
