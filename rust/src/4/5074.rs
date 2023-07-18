use std::io::stdin;

fn parse(time: &str) -> (i32, i32) {
    let mut iter = time.split(':');
    let h = iter.next().unwrap().parse().unwrap();
    let m = iter.next().unwrap().parse().unwrap();
    (h, m)
}

fn tick() -> bool {
    // get times
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    let mut iter = line.trim().split_whitespace();
    let mut time1 = parse(iter.next().unwrap());
    let time2 = parse(iter.next().unwrap());

    // terminal condition
    if (time1 == (0, 0)) && (time2 == (0, 0)) {
        return false;
    }

    // calculate
    time1.0 += time2.0;
    time1.1 += time2.1;

    time1.0 += time1.1 / 60;
    time1.1 %= 60;

    let days = time1.0 / 24;
    time1.0 %= 24;

    // print
    print!("{:02}:{:02}", time1.0, time1.1);
    if days > 0 {
        print!(" +{}", days);
    }
    println!();

    return true;
}

fn main() {
    while tick() {}
}
