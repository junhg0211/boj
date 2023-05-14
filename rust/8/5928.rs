use std::io;

fn calculate_minutes(date: i32, hour: i32, minute: i32) -> i32 {
    return minute + 60 * (hour + 24 * (date - 11));
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("failed to convert into integer"))
        .collect();

    let assign_time = calculate_minutes(numbers[0], numbers[1], numbers[2]);
    let start_time = calculate_minutes(11, 11, 11);

    let delta = assign_time - start_time;

    if delta < 0 {
        println!("-1");
    } else {
        println!("{}", delta);
    }
}
