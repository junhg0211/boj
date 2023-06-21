use std::io::stdin;

fn main() {
    // get numbers
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // fetch numbers
    let target = numbers[0];
    let min = numbers[1];
    let max = numbers[2];
    let training = numbers[3];
    let rest = numbers[4];

    // impossible guard
    if (max - min) / training < 1 {
        println!("-1");
        return;
    }

    // simulate
    let mut count = 0;
    let mut now = min;
    let mut time = 0;
    while count < target {
        if now + training <= max {
            count += 1;
            now += training;
        } else {
            now -= rest;
            if now < min {
                now = min;
            }
        }
        time += 1;
    }

    // print result
    println!("{}", time);
}
