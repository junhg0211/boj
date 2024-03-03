use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let line_count = input.trim().parse::<usize>().unwrap();

    let mut points = Vec::new();
    for _ in 0..line_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap());

        let start = iter.next().unwrap();
        let end = iter.next().unwrap();

        points.push((start, true));
        points.push((end, false));
    }
    points.sort();

    let mut result = 0;
    let mut value: i32 = 0;
    for (_, is_start) in points {
        if !is_start {
            value -= 1;
        } else {
            value += 1;
        }

        if value > result {
            result = value;
        }
    }

    println!("{}", result);
}
