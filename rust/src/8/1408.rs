use std::io::stdin;

fn get_time() -> (i8, i8, i8) {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut iter = input.trim().split(":");

    let hour = iter.next().unwrap().parse::<i8>().unwrap();
    let minute = iter.next().unwrap().parse::<i8>().unwrap();
    let second = iter.next().unwrap().parse::<i8>().unwrap();

    (hour, minute, second)
}

fn main() {
    // get times
    let time1 = get_time();
    let time2 = get_time();

    // calculate time difference
    let mut diff = (time2.0 - time1.0, time2.1 - time1.1, time2.2 - time1.2);
    while diff.2 < 0 {
        diff.2 += 60;
        diff.1 -= 1;
    }
    while diff.1 < 0 {
        diff.1 += 60;
        diff.0 -= 1;
    }
    while diff.0 < 0 {
        diff.0 += 24;
    }

    println!("{:02}:{:02}:{:02}", diff.0, diff.1, diff.2);
}
