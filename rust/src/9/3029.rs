use std::io::stdin;

fn get_time() -> (i32, i32, i32) {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let numbers = input
        .trim()
        .split(":")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    (numbers[0], numbers[1], numbers[2])
}

fn subtract(mut a: (i32, i32, i32), b: (i32, i32, i32)) -> (i32, i32, i32) {
    if a.2 < b.2 {
        a.2 += 60;
        a.1 -= 1;
    }
    let seconds = a.2 - b.2;

    if a.1 < b.1 {
        a.1 += 60;
        a.0 -= 1;
    }
    let minutes = a.1 - b.1;

    if a.0 < b.0 {
        a.0 += 24;
    }
    let hours = a.0 - b.0;

    if hours == 0 && minutes == 0 && seconds == 0 {
        (24, 0, 0)
    } else {
        (hours, minutes, seconds)
    }
}

fn main() {
    let result = {
        let now = get_time();
        let sodium = get_time();
        subtract(sodium, now)
    };

    println!("{:02}:{:02}:{:02}", result.0, result.1, result.2);
}
